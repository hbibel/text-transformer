use std::collections::HashMap;
use std::fmt::Display;

use crate::ast;
use crate::tokens;

// Note: I've not implemented function definitions yet. If and when I do that,
// I need to implement function calls instead of unwinding the operations,
// since as of right now recursion will lead to OOM on compilation.

pub fn compile(source_code: &str) -> Result<Program, CompileError> {
    let tokens = tokens::scan(source_code).map_err(|e| CompileError::ScanError { msg: e })?;
    let ast = ast::parse(tokens).map_err(|e| CompileError::SyntaxError { msg: e })?;
    Program::from_ast(&ast)
}

pub struct Program {
    ops: Vec<Op>,
}

struct ProgramState {
    output: String,
    stack: Vec<Value>,
}

impl Program {
    fn from_ast(ast: &ast::AST) -> Result<Program, CompileError> {
        let functions = builtin_functions();

        ast.statements
            .iter()
            .try_fold(Program { ops: vec![] }, |mut acc, stmt| match stmt {
                ast::Statement::FunctionCall { function_call } => {
                    let function_ops = compile_function_call(function_call, &functions)?;
                    acc.ops.extend(function_ops.ops);
                    Ok(acc)
                }
            })
    }

    pub fn run(&self, input: &str) -> Result<String, RuntimeError> {
        let init_state = ProgramState {
            output: String::new(),
            stack: Vec::new(),
        };
        let final_state =
            self.ops
                .iter()
                .try_fold(init_state, |mut program_state, op| match op {
                    Op::Print => match program_state.stack.pop() {
                        None => Err(RuntimeError {
                            msg: "Empty stack".to_string(),
                        }),
                        Some(val) => {
                            program_state.output.push_str(&format!("{val}"));
                            Ok(program_state)
                        }
                    },
                    Op::LoadItem => {
                        program_state.stack.push(Value::String(input.to_string()));
                        Ok(program_state)
                    }
                    Op::LoadIndex { index } => {
                        let list = program_state.stack.pop().ok_or(RuntimeError {
                            msg: "Empty stack".to_string(),
                        })?;
                        let elem = match list {
                            Value::List(l) => l
                                .get(*index as usize)
                                .ok_or(RuntimeError {
                                    msg: "out of bounds".to_string(),
                                })
                                .cloned(),
                            _ => Err(RuntimeError {
                                msg: "not a list".to_string(),
                            }),
                        }?;
                        program_state.stack.push(Value::String(elem));
                        Ok(program_state)
                    }
                    Op::SplitStr => {
                        let s = program_state.stack.pop().ok_or(RuntimeError {
                            msg: "Empty stack".to_string(),
                        })?;
                        let s = match s {
                            Value::List(_) => Err(RuntimeError {
                                msg: "Expected String".to_string(),
                            }),
                            Value::String(s) => Ok(s),
                        }?;
                        program_state.stack.push(Value::List(
                            s.split_whitespace().map(|s| s.to_string()).collect(),
                        ));
                        Ok(program_state)
                    }
                })?;
        Ok(final_state.output)
    }

    // TODO build binary for disassembling the program, for debugging
    // fn disassemble(&self) -> String {
    //     self.ops.iter().fold(String::new(), |mut acc, op| {
    //         acc.push_str(&match op {
    //             Op::Print => "PRINT\n".to_string(),
    //             Op::LoadItem => "LOAD_ITEM\n".to_string(),
    //             Op::LoadIndex { index } => format!("LOAD_INDEX {}", index),
    //         });
    //         acc
    //     })
    // }
}

fn builtin_functions() -> HashMap<String, TypedFunction> {
    let mut map = HashMap::new();

    map.insert(
        "print".to_string(),
        TypedFunction {
            ops: vec![Op::Print],
            return_type: Type::Unit,
            arg_types: vec![Type::String],
        },
    );

    map.insert("split".to_string(), {
        TypedFunction {
            ops: vec![Op::SplitStr],
            return_type: Type::List,
            arg_types: vec![Type::String],
        }
    });

    map
}

fn compile_expr(
    expr: &ast::Expr,
    functions: &HashMap<String, TypedFunction>,
) -> Result<TypedComputation, CompileError> {
    let ast::Expr { arr_expr, index } = expr;
    let mut comp = match arr_expr {
        ast::ArrExpr::ValueExpr { value } => match value {
            ast::Value::Item => TypedComputation {
                ops: vec![Op::LoadItem],
                result_type: Type::String,
            },
            ast::Value::Identifier(_id) => todo!(),
        },
        ast::ArrExpr::FunctionExpr { function_call } => {
            compile_function_call(function_call, functions)?
        }
    };

    // if index is not None => assert result is array, then index it
    match (*index, &comp.result_type) {
        (Some(index), Type::List) => {
            comp.ops.push(Op::LoadIndex { index });
            comp.result_type = Type::String;
            Ok(comp)
        }
        (None, _) => Ok(comp),
        (Some(_), other_type) => Err(CompileError::SemanticAnalysisError {
            msg: format!("Expected {}, got {}", Type::List, other_type),
        }),
    }
}

fn compile_function_call(
    function_call: &ast::FunctionCall,
    functions: &HashMap<String, TypedFunction>,
) -> Result<TypedComputation, CompileError> {
    let mut ops = Vec::new();

    let ast::FunctionCall {
        function_name,
        args,
    } = function_call;

    let TypedFunction {
        ops: function_ops,
        return_type,
        arg_types,
    } = functions
        .get(function_name)
        .ok_or(CompileError::SemanticAnalysisError {
            msg: format!("Function {} not found", function_name),
        })?;

    let arg_computations: Vec<TypedComputation> = args
        .iter()
        .map(|arg| compile_expr(arg, functions))
        .collect::<Result<Vec<_>, CompileError>>()?;
    if args.len() != arg_types.len() {
        let msg = format!(
            "Function {} takes {} arguments, but {} were given",
            function_name,
            arg_types.len(),
            arg_computations.len()
        );
        return Err(CompileError::SemanticAnalysisError { msg });
    }
    for (i, (arg_c, exp_type)) in arg_computations.iter().zip(arg_types).enumerate() {
        if arg_c.result_type != *exp_type {
            let msg = format!(
                "Argument {} to function {} is wrong: Expected {}, got {}",
                i, function_name, exp_type, arg_c.result_type
            );
            return Err(CompileError::SemanticAnalysisError { msg });
        }
    }

    let arg_ops: Vec<Op> = arg_computations
        .iter()
        .flat_map(|c| c.ops.clone())
        .collect();
    ops.extend(arg_ops);
    ops.extend(function_ops.to_owned());

    Ok(TypedComputation {
        ops,
        result_type: return_type.clone(),
    })
}

#[derive(Debug)]
pub struct RuntimeError {
    msg: String,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
pub enum CompileError {
    ScanError { msg: String },
    SyntaxError { msg: String },
    SemanticAnalysisError { msg: String },
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::ScanError { msg } => write!(f, "{}", msg),
            CompileError::SyntaxError { msg } => write!(f, "{}", msg),
            CompileError::SemanticAnalysisError { msg } => write!(f, "{}", msg),
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Print,
    LoadItem,
    LoadIndex { index: i32 },
    SplitStr,
}

#[derive(Debug, Clone)]
enum Value {
    String(String),
    List(Vec<String>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::List(ss) => ss.iter().try_for_each(|s| write!(f, "{}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Unit,
    String,
    List,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => write!(f, "Unit"),
            Type::String => write!(f, "String"),
            Type::List => write!(f, "List"),
        }
    }
}

#[derive(Debug)]
struct TypedFunction {
    ops: Vec<Op>,
    return_type: Type,
    arg_types: Vec<Type>,
}

#[derive(Debug)]
struct TypedComputation {
    ops: Vec<Op>,
    result_type: Type,
}

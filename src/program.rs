#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fmt::Display;

use crate::ast;
use crate::tokens;

// Note: I've not implemented function definitions yet. If and when I do that,
// I need to implement function calls instead of unwinding the operations,
// since as of right now recursion will lead to OOM on compilation.

pub fn compile(source_code: String) -> Result<Program, CompileError> {
    let tokens = tokens::scan(source_code).map_err(|e| CompileError { msg: e })?;
    let ast = ast::parse(tokens).map_err(|e| CompileError { msg: e })?;
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
                            program_state.output.push_str(&format!("{}", val));
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
                })?;
        Ok(final_state.output)
    }

    fn disassemble(&self) -> String {
        self.ops.iter().fold(String::new(), |mut acc, op| {
            acc.push_str(&match op {
                Op::Print => "PRINT\n".to_string(),
                Op::LoadItem => "LOAD_ITEM\n".to_string(),
                Op::LoadIndex { index } => format!("LOAD_INDEX {}", index),
            });
            acc
        })
    }
}

fn builtin_functions() -> HashMap<String, TypedOps> {
    let mut map = HashMap::new();

    map.insert(
        "print".to_string(),
        TypedOps {
            ops: vec![Op::Print],
            tpe: Type::Unit,
        },
    );

    map
}

fn compile_expr(
    expr: &ast::Expr,
    functions: &HashMap<String, TypedOps>,
) -> Result<Vec<Op>, CompileError> {
    let ast::Expr { arr_expr, index } = expr;
    let TypedOps { mut ops, tpe } = match arr_expr {
        ast::ArrExpr::ValueExpr { value } => match value {
            ast::Value::Item => TypedOps {
                ops: vec![Op::LoadItem],
                tpe: Type::String,
            },
            ast::Value::Identifier(id) => todo!(),
        },
        ast::ArrExpr::FunctionExpr { function_call } => {
            compile_function_call(function_call, functions)?
        }
    };

    // if index is not None => assert result is array, then index it
    match (*index, tpe) {
        (Some(index), Type::List) => {
            ops.push(Op::LoadIndex { index });
            Ok(ops)
        }
        (None, _) => Ok(ops),
        (Some(_), other_type) => Err(CompileError {
            msg: format!("Expected list, got {}", other_type),
        }),
    }
}

fn compile_function_call(
    function_call: &ast::FunctionCall,
    functions: &HashMap<String, TypedOps>,
) -> Result<TypedOps, CompileError> {
    let mut ops = Vec::new();

    let ast::FunctionCall {
        function_name,
        args,
    } = function_call;

    let arg_ops = args
        .iter()
        .map(|arg| compile_expr(arg, functions))
        .collect::<Result<Vec<Vec<Op>>, CompileError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Op>>();
    ops.extend(arg_ops);

    let TypedOps {
        ops: function_ops,
        tpe,
    } = functions.get(function_name).ok_or(CompileError {
        msg: format!("Function {} not found", function_name),
    })?;
    ops.extend(function_ops.to_owned());

    Ok(TypedOps {
        ops,
        tpe: tpe.clone(),
    })
}

#[derive(Debug)]
pub struct RuntimeError {
    msg: String,
}

#[derive(Debug)]
pub struct CompileError {
    msg: String,
}

#[derive(Debug, Clone)]
enum Op {
    Print,
    LoadItem,
    LoadIndex { index: i32 },
}

#[derive(Debug)]
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

#[derive(Debug, Clone)]
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
struct TypedOps {
    ops: Vec<Op>,
    tpe: Type,
}

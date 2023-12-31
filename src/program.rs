use std::collections::HashMap;

use crate::ast;
use crate::tokens;

/// Maps item to a unit result
struct Statement {
    fc: ast::FunctionCall,
}

impl Statement {
    fn apply(&self, item: &String, functions: &HashMap<String, Function>) -> Result<(), String> {
        eval(&self.fc, item, functions).map(|_| ())
    }
}

type Evaluate = Box<dyn Fn(&Vec<String>) -> Result<String, String>>;

pub struct Program {
    statements: Vec<Statement>,
    functions: HashMap<String, Function>,
}

struct Function {
    pub num_args: i32,
    pub evaluate: Evaluate,
}

impl Program {
    pub fn run(&self, input: &String) -> Result<(), String> {
        self.statements
            .iter()
            .try_for_each(|s| s.apply(input, &self.functions))
    }
}

pub fn compile(source_code: String) -> Result<Program, String> {
    tokens::scan(source_code)
        .and_then(ast::parse)
        .and_then(to_program)
}

fn to_program(ast: ast::AST) -> Result<Program, String> {
    // Currently just a tree-walking implementation
    use ast::Expression::*;

    let functions: HashMap<String, Function> = builtin_functions();

    let statements: Vec<Statement> = ast
        .expressions
        .iter()
        .map(|e| match e {
            FunctionExpr(fc) => Statement { fc: fc.clone() },
            // TODO remove branch after removing top-level values from syntax
            _ => panic!("Should not be implemented"),
        })
        .collect();
    Ok(Program {
        statements,
        functions,
    })
}

fn eval(
    fcall: &ast::FunctionCall,
    item: &String,
    functions: &HashMap<String, Function>,
) -> Result<String, String> {
    let ast::FunctionCall {
        function_name,
        args,
    } = fcall;
    let f = functions
        .get(function_name)
        .ok_or(format!("Function {} not found", function_name))?;
    if f.num_args != args.len() as i32 {
        return Err(format!(
            "Function {} expects {} arguments, but {} were provided",
            function_name,
            f.num_args,
            args.len()
        ));
    }

    let arg_results = args
        .iter()
        .flat_map(|a| match a {
            ast::Expression::ValueExpr(val) => match val {
                ast::Value::Item => Ok(item.clone()),
                ast::Value::Number(x) => Ok(x.clone()),
                ast::Value::Identifier(_x) => Ok(String::from("todo")),
            },
            ast::Expression::FunctionExpr(fcall) => eval(fcall, item, functions),
        })
        .collect::<Vec<String>>();
    (f.evaluate)(&arg_results)
}

fn builtin_functions() -> HashMap<String, Function> {
    HashMap::from([(
        String::from("print"),
        Function {
            num_args: 1,
            evaluate: Box::new(|args| {
                println!("{}", args[0]);
                Ok(String::from(""))
            }),
        },
    )])
}

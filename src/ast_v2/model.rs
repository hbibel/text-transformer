// TODO temporarily disabling warnings
#![allow(dead_code)]

pub use super::expression::Expression;
pub use super::function_call::FunctionCall;
pub use super::value::Value;

pub struct Ast {
    pub root: Program,
}

pub struct Program {
    statements: Vec<Statement>,
    prologue: Vec<Statement>,
    epilogue: Vec<Statement>,
}

pub enum Statement {
    Fc(FunctionCall),
    As(Assignment),
    Fd(FunctionDefinition),
}

pub struct Assignment {
    item_name: String,
    expression: Expression,
}

pub struct FunctionDefinition {
    function_name: String,
    arguments: Vec<String>,
    body: Vec<Statement>,
}

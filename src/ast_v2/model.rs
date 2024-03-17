// TODO temporarily disabling warnings
#![allow(dead_code)]

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

#[derive(PartialEq, Eq, Debug)]
pub struct FunctionCall {
    function_name: String,
    args: Vec<Expression>,
}

pub struct Assignment {
    assignee: String,
    expression: Expression,
}

pub struct FunctionDefinition {
    function_name: String,
    arguments: Vec<String>,
    body: Vec<Statement>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    Fc(FunctionCall),
    Val(Value),
    IndexedExpression(Box<Expression>, usize),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Value {
    Item,
    Identifier(String),
    LiteralString(String),
    LiteralInteger(i64),
}

// TODO temporarily disabling warnings
#![allow(dead_code)]

#[derive(PartialEq, Eq, Debug)]
pub struct Ast {
    pub root: Program,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub prologue: Vec<Statement>,
    pub epilogue: Vec<Statement>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    Fc(FunctionCall),
    As(Assignment),
    Fd(FunctionDefinition),
}

#[derive(PartialEq, Eq, Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub args: Vec<Expression>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Assignment {
    pub assignee: String,
    pub expression: Expression,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FunctionDefinition {
    pub function_name: String,
    pub arguments: Vec<String>,
    pub body: Vec<Statement>,
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

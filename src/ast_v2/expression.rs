// TODO allow
#![allow(dead_code)]
use super::{function_call::FunctionCall, value::Value};
use crate::tokens::Token;

#[derive(PartialEq, Eq, Debug)]

pub enum Expression {
    Fc(FunctionCall),
    Val(Value),
    IndexedExpression(Box<Expression>, usize),
}

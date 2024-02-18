// TODO allow
#![allow(dead_code)]

use super::expression::Expression;
use crate::{
    ast_v2::{identifier, utils::parse_constant_token},
    tokens::Token,
};

#[derive(PartialEq, Eq, Debug)]
pub struct FunctionCall {
    function_name: String,
    args: Vec<Expression>,
}

pub fn parse(tokens: &[Token]) -> Option<(FunctionCall, &[Token])> {
    let tokens = parse_constant_token(tokens, &Token::Alphanum("fn".to_string()))?;
    let (f_name, tokens) = identifier::parse(tokens)?;
    let tokens = parse_constant_token(tokens, &Token::OpenParen)?;
    let (args, tokens) = parse_arguments(tokens)?;
    let tokens = parse_constant_token(tokens, &Token::CloseParen)?;
    todo!()
}

fn parse_arguments(tokens: &[Token]) -> Option<(Vec<Expression>, &[Token])> {
    let args = Vec::new();
    match tokens.first() {
        Some(Token::CloseParen) => Some((Vec::new(), tokens)),
        _ => todo!(),
    }
}

// TODO test circular

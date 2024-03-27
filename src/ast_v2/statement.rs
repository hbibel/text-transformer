use crate::{
    ast_v2::{model::Assignment, utils::parse_constant_token},
    tokens::Token,
};

use super::model::{FunctionCall, FunctionDefinition, Statement};

impl Statement {
    pub fn parse(tokens: &[Token]) -> Option<(Self, &[Token])> {
        let as_function_call = FunctionCall::parse(tokens).map(|(fc, ts)| (Self::Fc(fc), ts));
        let as_assignment = || Assignment::parse(tokens).map(|(asgn, ts)| (Self::As(asgn), ts));
        let as_function_definition =
            || FunctionDefinition::parse(tokens).map(|(fd, ts)| (Self::Fd(fd), ts));

        as_function_call
            .or_else(as_assignment)
            .or_else(as_function_definition)
    }

    pub fn parse_block(tokens: &[Token]) -> Option<(Vec<Self>, &[Token])> {
        let mut tokens = parse_constant_token(tokens, &Token::OpenBrace)?;

        let mut statments = Vec::new();

        // parse first statement
        if let Some((stmt, ts)) = Self::parse(tokens) {
            tokens = ts;
            statments.push(stmt);
        }

        while parse_constant_token(tokens, &Token::CloseBrace).is_none() {
            // all further statements need to be separated by semicolons
            tokens = parse_constant_token(tokens, &Token::Semicolon)?;
            let (stmt, ts) = Self::parse(tokens)?;
            tokens = ts;
            statments.push(stmt);
        }

        let tokens = parse_constant_token(tokens, &Token::CloseBrace)?;
        Some((statments, tokens))
    }
}

use crate::{
    ast_v2::utils::{parse_constant_alphanum, parse_constant_token},
    tokens::Token,
};

use super::model::{Program, Statement};

impl Program {
    pub fn parse(tokens: &[Token]) -> Option<Program> {
        let mut tokens = tokens;
        let mut prologue = Vec::new();
        if let Some(ts) = parse_constant_alphanum(tokens, "prologue") {
            tokens = ts;
            (prologue, tokens) = Statement::parse_block(tokens)?;
        }

        let mut statements = Vec::new();

        // parse first statement
        if let Some((stmt, ts)) = Statement::parse(tokens) {
            tokens = ts;
            statements.push(stmt);
        }

        while parse_constant_alphanum(tokens, "epilogue").is_none() {
            // all further statements need to be separated by semicolons
            tokens = parse_constant_token(tokens, &Token::Semicolon)?;
            let (stmt, ts) = Statement::parse(tokens)?;
            tokens = ts;
            statements.push(stmt);
        }

        let mut epilogue = Vec::new();
        if let Some(ts) = parse_constant_alphanum(tokens, "epilogue") {
            tokens = ts;
            (epilogue, tokens) = Statement::parse_block(tokens)?;
        }

        if tokens.is_empty() {
            Some(Program {
                statements,
                prologue,
                epilogue,
            })
        } else {
            None
        }
    }
}

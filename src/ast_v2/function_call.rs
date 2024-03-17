// TODO allow
#![allow(dead_code)]

use crate::tokens::Token;

use super::model::{Expression, FunctionCall};
use super::{identifier, utils::parse_constant_token};

impl FunctionCall {
    pub fn parse(tokens: &[Token]) -> Option<(FunctionCall, &[Token])> {
        let (function_name, tokens) = identifier::parse(tokens)?;
        let tokens = parse_constant_token(tokens, &Token::OpenParen)?;
        let (args, tokens) = parse_arguments(tokens)?;
        let tokens = parse_constant_token(tokens, &Token::CloseParen)?;

        Some((
            FunctionCall {
                function_name,
                args,
            },
            tokens,
        ))
    }
}

fn parse_arguments(mut tokens: &[Token]) -> Option<(Vec<Expression>, &[Token])> {
    let mut args: Vec<Expression> = Vec::new();
    while tokens.first() != Some(&Token::CloseParen) {
        let (expr, ts) = Expression::parse(tokens)?;
        tokens = ts;
        if tokens.first() != Some(&Token::CloseParen) {
            tokens = parse_constant_token(tokens, &Token::Comma)?;
        }
        args.push(expr);
    }
    Some((args, tokens))
}

#[cfg(test)]
mod tests {
    use crate::ast_v2::model::Value;

    use super::*;

    #[test]
    fn test_no_args() {
        let tokens = vec![
            Token::Alphanum("foo".to_string()),
            Token::OpenParen,
            Token::CloseParen,
        ];
        let actual = FunctionCall::parse(&tokens).unwrap();
        let expected = (
            FunctionCall {
                function_name: "foo".to_string(),
                args: vec![],
            },
            &[] as &[Token],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_one_arg() {
        let tokens = vec![
            Token::Alphanum("foo".to_string()),
            Token::OpenParen,
            Token::Alphanum("42".to_string()),
            Token::CloseParen,
        ];
        let actual = FunctionCall::parse(&tokens).unwrap();
        let expected = (
            FunctionCall {
                function_name: "foo".to_string(),
                args: vec![Expression::Val(Value::LiteralInteger(42))],
            },
            &[] as &[Token],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiple_args() {
        let tokens = vec![
            Token::Alphanum("foo".to_string()),
            Token::OpenParen,
            Token::Alphanum("42".to_string()),
            Token::Comma,
            Token::Alphanum("84".to_string()),
            Token::CloseParen,
        ];
        let actual = FunctionCall::parse(&tokens).unwrap();
        let expected = (
            FunctionCall {
                function_name: "foo".to_string(),
                args: vec![
                    Expression::Val(Value::LiteralInteger(42)),
                    Expression::Val(Value::LiteralInteger(84)),
                ],
            },
            &[] as &[Token],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_nested_function_call() {
        let tokens = vec![
            Token::Alphanum("foo".to_string()),
            Token::OpenParen,
            Token::Alphanum("42".to_string()),
            Token::CloseParen,
        ];
        let actual = FunctionCall::parse(&tokens).unwrap();
        let expected = (
            FunctionCall {
                function_name: "foo".to_string(),
                args: vec![Expression::Val(Value::LiteralInteger(42))],
            },
            &[] as &[Token],
        );
        assert_eq!(actual, expected);
    }
}

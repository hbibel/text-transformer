// TODO temporarily disabling warnings
#![allow(dead_code)]

use crate::tokens::Token;

use super::{
    identifier,
    model::{FunctionDefinition, Statement},
    utils::{parse_constant_alphanum, parse_constant_token},
};

impl FunctionDefinition {
    pub fn parse(tokens: &[Token]) -> Option<(Self, &[Token])> {
        let tokens = parse_constant_alphanum(tokens, "fn")?;

        let (function_name, tokens) = identifier::parse(tokens)?;

        let mut tokens = parse_constant_token(tokens, &Token::OpenParen)?;
        let mut arguments = Vec::new();
        while let Some((arg_name, ts)) = identifier::parse(tokens) {
            tokens = ts;
            arguments.push(arg_name);
        }
        let tokens = parse_constant_token(tokens, &Token::CloseParen)?;

        let (body, tokens) = Statement::parse_block(tokens)?;

        Some((
            FunctionDefinition {
                function_name,
                arguments,
                body,
            },
            tokens,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_args() {
        let tokens = vec![
            Token::Alphanum("fn".to_string()),
            Token::Alphanum("foo".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
        ];
        let expected = FunctionDefinition {
            function_name: "foo".to_string(),
            arguments: Vec::new(),
            body: Vec::new(),
        };
        let (actual, remaining_tokens) = FunctionDefinition::parse(&tokens).unwrap();
        assert!(remaining_tokens.is_empty());
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_one_arg() {
        let tokens = vec![
            Token::Alphanum("fn".to_string()),
            Token::Alphanum("foo".to_string()),
            Token::OpenParen,
            Token::Alphanum("bar".to_string()),
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
        ];
        let expected = FunctionDefinition {
            function_name: "foo".to_string(),
            arguments: vec!["bar".to_string()],
            body: Vec::new(),
        };
        let (actual, remaining_tokens) = FunctionDefinition::parse(&tokens).unwrap();
        assert!(remaining_tokens.is_empty());
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_multiple_args() {
        let tokens = vec![
            Token::Alphanum("fn".to_string()),
            Token::Alphanum("foo".to_string()),
            Token::OpenParen,
            Token::Alphanum("bar".to_string()),
            Token::Alphanum("baz".to_string()),
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
        ];
        let expected = FunctionDefinition {
            function_name: "foo".to_string(),
            arguments: vec!["bar".to_string(), "baz".to_string()],
            body: Vec::new(),
        };
        let (actual, remaining_tokens) = FunctionDefinition::parse(&tokens).unwrap();
        assert!(remaining_tokens.is_empty());
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_nested() {
        // fn foo() { fn bar() {} }
        let tokens = vec![
            Token::Alphanum("fn".to_string()),
            Token::Alphanum("foo".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::Alphanum("fn".to_string()),
            Token::Alphanum("bar".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::CloseBrace,
        ];
        let expected = FunctionDefinition {
            function_name: "foo".to_string(),
            arguments: Vec::new(),
            body: vec![Statement::Fd(FunctionDefinition {
                function_name: "bar".to_string(),
                arguments: Vec::new(),
                body: Vec::new(),
            })],
        };
        let (actual, remaining_tokens) = FunctionDefinition::parse(&tokens).unwrap();
        assert!(remaining_tokens.is_empty());
        assert_eq!(actual, expected);
    }
}

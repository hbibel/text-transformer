// TODO allow
#![allow(dead_code)]

use crate::tokens::Token;

use super::{identifier, model::Value};

pub fn parse(tokens: &[Token]) -> Option<(Value, &[Token])> {
    parse_item(tokens)
        .or_else(|| parse_identified(tokens))
        .or_else(|| parse_integer(tokens))
    // .or_else(|| parse_string(tokens))
}

fn parse_item(tokens: &[Token]) -> Option<(Value, &[Token])> {
    match tokens.first() {
        Some(Token::Underscore) => Some((Value::Item, &tokens[1..])),
        other => {
            println!("Token: {other:?}");
            None
        }
    }
}

fn parse_identified(tokens: &[Token]) -> Option<(Value, &[Token])> {
    identifier::parse(tokens).map(|(ident, rem)| (Value::Identifier(ident), rem))
}

fn parse_integer(tokens: &[Token]) -> Option<(Value, &[Token])> {
    match tokens.first() {
        Some(Token::Alphanum(s)) => s
            .parse()
            .map(|i| (Value::LiteralInteger(i), &tokens[1..]))
            .ok(),
        _ => None,
    }
}

fn parse_string(_tokens: &[Token]) -> Option<(Value, &[Token])> {
    // TODO there is no Token::String yet
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // parse result type
    type R<'a> = Option<(Value, &'a [Token])>;

    fn unrelated_token() -> Token {
        Token::OpenBracket
    }

    #[test]
    fn parse_empty() {
        let input = &[];
        let expected = None;
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_unrelated_token() {
        let input = &[unrelated_token()];
        let expected = None;
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_unrelated_token_followed_by_item() {
        let input = &[unrelated_token(), Token::Underscore];
        let expected = None;
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_item() {
        let input = &[Token::Underscore];
        let expected: R = Some((Value::Item, &[]));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_item_remainder() {
        let input = &[Token::Underscore, unrelated_token()];
        let expected_remaining = [unrelated_token()];
        let expected: R = Some((Value::Item, &expected_remaining));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_identifier() {
        let input = &[Token::Alphanum("foo".to_string())];
        let expected: R = Some((Value::Identifier("foo".to_string()), &[]));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_identifier_with_digits() {
        let input = &[Token::Alphanum("foo1".to_string())];
        let expected: R = Some((Value::Identifier("foo1".to_string()), &[]));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_identifier_remainder() {
        let input = &[Token::Alphanum("foo".to_string()), unrelated_token()];
        let expected_remaining = [unrelated_token()];
        let expected: R = Some((Value::Identifier("foo".to_string()), &expected_remaining));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_integer() {
        let input = &[Token::Alphanum("123".to_string())];
        let expected: R = Some((Value::LiteralInteger(123), &[]));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_integer_remainder() {
        let input = &[Token::Alphanum("123".to_string()), unrelated_token()];
        let expected_remaining = [unrelated_token()];
        let expected: R = Some((Value::LiteralInteger(123), &expected_remaining));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    fn parse_unrelated_alphanum() {
        let input = &[Token::Alphanum("1xy".to_string()), unrelated_token()];
        let expected = None;
        let actual = parse(input);
        assert_eq!(actual, expected);
    }
}

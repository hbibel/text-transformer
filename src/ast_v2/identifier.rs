use crate::tokens::Token;

pub fn parse(tokens: &[Token]) -> Option<(String, &[Token])> {
    match tokens.first() {
        Some(Token::Alphanum(s)) if s.chars().next().is_some_and(char::is_alphabetic) => {
            Some((s.clone(), &tokens[1..]))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // parse result type
    type R<'a> = Option<(String, &'a [Token])>;

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
    fn parse_alphabetic() {
        let input = &[Token::Alphanum("foo".to_string())];
        let expected: R = Some(("foo".to_string(), &[]));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_alphanumeric() {
        let input = &[Token::Alphanum("foo1".to_string())];
        let expected: R = Some(("foo1".to_string(), &[]));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_with_remaining_tokens() {
        let input = &[Token::Alphanum("foo".to_string()), unrelated_token()];
        let expected_remaining = [unrelated_token()];
        let expected: R = Some(("foo".to_string(), &expected_remaining));
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_alphanumeric_starting_with_digit() {
        let input = &[Token::Alphanum("1foo".to_string())];
        let expected = None;
        let actual = parse(input);
        assert_eq!(actual, expected);
    }
}

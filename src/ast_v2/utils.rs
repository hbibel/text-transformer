use crate::tokens::Token;

pub fn parse_constant_token<'a>(tokens: &'a [Token], expected: &Token) -> Option<&'a [Token]> {
    match tokens.first() {
        Some(t) if t == expected => Some(&tokens[1..]),
        _ => None,
    }
}

pub fn parse_constant_alphanum<'a>(tokens: &'a [Token], expected: &str) -> Option<&'a [Token]> {
    match tokens.first() {
        Some(Token::Alphanum(actual)) if actual == expected => Some(&tokens[1..]),
        _ => None,
    }
}

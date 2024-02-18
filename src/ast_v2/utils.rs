use crate::tokens::Token;

pub fn parse_constant_token<'a>(tokens: &'a [Token], expected: &Token) -> Option<&'a [Token]> {
    match tokens.first() {
        Some(expected) => Some(&tokens[1..]),
        _ => None,
    }
}

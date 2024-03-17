use crate::tokens::Token;

use super::{
    model::{Expression, FunctionCall, Value},
    utils::parse_constant_token,
};

impl Expression {
    pub fn parse(tokens: &[Token]) -> Option<(Expression, &[Token])> {
        let (mut expr, mut tokens) = {
            let maybe_as_value = Value::parse(tokens);
            if let Some((v, ts)) = maybe_as_value {
                Some((Expression::Val(v), ts))
            } else {
                FunctionCall::parse(tokens).map(|(fc, ts)| (Expression::Fc(fc), ts))
            }
        }?;
        // Check if an indexing clause, like [123], follows
        if tokens.first() == Some(&Token::OpenBracket) {
            tokens = parse_constant_token(tokens, &Token::OpenBracket)?;
            let (i_value, ts) = Value::parse(tokens)?;
            tokens = ts;
            let index = match i_value {
                Value::LiteralInteger(i) => usize::try_from(i).ok(),
                _ => None,
            }?;
            expr = Expression::IndexedExpression(Box::new(expr), index);
            tokens = parse_constant_token(tokens, &Token::CloseBracket)?;
        }
        Some((expr, tokens))
    }
}

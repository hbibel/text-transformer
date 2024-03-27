use crate::{ast_v2::model::Assignment, tokens::Token};

use super::{identifier, model::Expression, utils::parse_constant_token};

impl Assignment {
    pub fn parse(tokens: &[Token]) -> Option<(Assignment, &[Token])> {
        let (assignee, tokens) = identifier::parse(tokens)?;

        let tokens = parse_constant_token(tokens, &Token::EqualSign)?;

        let (expression, tokens) = Expression::parse(tokens)?;

        Some((
            Assignment {
                assignee,
                expression,
            },
            tokens,
        ))
    }
}

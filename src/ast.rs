use crate::tokens::Token;

#[derive(Debug, PartialEq)]
pub struct AST {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    FunctionCall { function_call: FunctionCall },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub function_name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub arr_expr: ArrExpr,
    pub index: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrExpr {
    FunctionExpr { function_call: FunctionCall },
    ValueExpr { value: Value },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Item, // _
    Identifier(String),
    // String(String),
}

// TODO no syntax error handling yet; see
// https://craftinginterpreters.com/parsing-expressions.html#syntax-errors
pub fn parse(tokens: Vec<Token>) -> Result<AST, String> {
    let mut token_slice = &tokens[..];
    let mut statements = Vec::new();

    while !token_slice.is_empty() {
        let (statement, remainder) = parse_statement(token_slice)?;
        log::trace!("parsed statement {:?}", statement);
        token_slice = remainder;
        statements.push(statement);

        if !token_slice.is_empty() {
            token_slice = expect_token(Token::Semicolon, token_slice)?;
        }
    }

    Ok(AST { statements })
}

fn parse_statement(tokens: &[Token]) -> Result<(Statement, &[Token]), String> {
    let (function_call, tokens) = parse_function_call(tokens)?;
    Ok((Statement::FunctionCall { function_call }, tokens))
}

fn parse_function_call(tokens: &[Token]) -> Result<(FunctionCall, &[Token]), String> {
    let (function_name, mut tokens) = parse_identifier(tokens)?;
    tokens = expect_token(Token::OpenParen, tokens)?;

    let mut expressions = vec![];
    let mut done;
    (done, tokens) = match expect_token(Token::CloseParen, tokens) {
        Ok(tail) => (true, tail),
        Err(_) => (false, tokens),
    };
    while !done {
        let expr;
        (expr, tokens) = parse_expr(tokens)?;
        expressions.push(expr);
        (done, tokens) = match expect_token(Token::CloseParen, tokens) {
            Ok(ts) => (true, ts),
            Err(_) => (false, tokens),
        };
    }

    Ok((
        FunctionCall {
            function_name,
            args: expressions,
        },
        tokens,
    ))
}

fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    let (expr, tokens) = parse_arr_expr(tokens)?;
    let (index, tokens) = match tokens {
        [Token::OpenBracket, ..] => parse_array_index(tokens).map(|(v, t)| (Some(v), t))?,
        _ => (None, tokens),
    };
    Ok((
        Expr {
            arr_expr: expr,
            index,
        },
        tokens,
    ))
}

fn parse_arr_expr(tokens: &[Token]) -> Result<(ArrExpr, &[Token]), String> {
    parse_function_call(tokens)
        .map(|(function_call, ts)| (ArrExpr::FunctionExpr { function_call }, ts))
        .or(parse_value(tokens).map(|(value, ts)| (ArrExpr::ValueExpr { value }, ts)))
}

fn expect_token(token: Token, tokens: &[Token]) -> Result<&[Token], String> {
    match tokens.get(0) {
        Some(t) if *t == token => Ok(&tokens[1..]),
        _ => Err(String::from(format!(
            "Expected char '{:?}' not found",
            token
        ))),
    }
}

fn parse_array_index(tokens: &[Token]) -> Result<(i32, &[Token]), String> {
    let tokens = expect_token(Token::OpenBracket, tokens)?;
    let (index, tokens) = match tokens.first() {
        Some(Token::Alphanum(s)) => s
            .parse::<i32>()
            .map_err(|err| err.to_string())
            .map(|r| (r, &tokens[1..])),
        Some(other) => Err(format!("Unexpected token")),
        None => Err("Missing index".to_string()),
    }?;
    let tokens = expect_token(Token::CloseBracket, tokens)?;
    Ok((index, tokens))
}

fn parse_value(tokens: &[Token]) -> Result<(Value, &[Token]), String> {
    parse_item(tokens).or(parse_identifier(tokens).map(|(i, ts)| (Value::Identifier(i), ts)))
}

fn parse_number(tokens: &[Token]) -> Result<(String, &[Token]), String> {
    match tokens.first() {
        Some(Token::Alphanum(s)) if s.chars().all(|c| char::is_digit(c, 10)) => {
            Ok((s.clone(), &tokens[1..]))
        }
        Some(_) => Err(String::from("Not a number")),
        None => Err(String::from("No token left")),
    }
}

fn parse_identifier(tokens: &[Token]) -> Result<(String, &[Token]), String> {
    match tokens.first() {
        Some(Token::Alphanum(s))
            if s.chars().nth(0).map(|c| c.is_alphabetic()).unwrap_or(false) =>
        {
            Ok((s.clone(), &tokens[1..]))
        }
        Some(_) => Err(String::from("Not an identifier")),
        None => Err(String::from("No token left")),
    }
}

fn parse_item(tokens: &[Token]) -> Result<(Value, &[Token]), String> {
    match tokens.first() {
        Some(Token::Underscore) => Ok((Value::Item, &tokens[1..])),
        _ => Err(String::from("Not an underscore")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_function_expr() {
        // foo(_)
        let tokens = vec![
            Token::Alphanum(String::from("foo")),
            Token::OpenParen,
            Token::Underscore,
            Token::CloseParen,
        ];
        let expected = AST {
            statements: vec![Statement::FunctionCall {
                function_call: FunctionCall {
                    function_name: String::from("foo"),
                    args: vec![Expr {
                        arr_expr: ArrExpr::ValueExpr { value: Value::Item },
                        index: None,
                    }],
                },
            }],
        };
        let actual = parse(tokens).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_with_array() {
        // foo(_[1])
        let tokens = vec![
            Token::Alphanum(String::from("foo")),
            Token::OpenParen,
            Token::Underscore,
            Token::OpenBracket,
            Token::Alphanum(String::from("1")),
            Token::CloseBracket,
            Token::CloseParen,
        ];
        let expected = AST {
            statements: vec![Statement::FunctionCall {
                function_call: FunctionCall {
                    function_name: String::from("foo"),
                    args: vec![Expr {
                        arr_expr: ArrExpr::ValueExpr { value: Value::Item },
                        index: Some(1),
                    }],
                },
            }],
        };
        let actual = parse(tokens).unwrap();
        assert_eq!(actual, expected);
    }
}

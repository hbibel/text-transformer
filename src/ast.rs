use crate::tokens::Token;

#[derive(Debug)]
pub struct AST {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    FunctionExpr(FunctionCall),
    ValueExpr(Value),
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function_name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Item, // _
    Identifier(String),
    // String(String),
    Number(String),
}
// TODO no syntax error handling yet; see
// https://craftinginterpreters.com/parsing-expressions.html#syntax-errors
pub fn parse(tokens: Vec<Token>) -> Result<AST, String> {
    let mut token_slice = &tokens[..];
    let mut expressions = Vec::new();

    while !token_slice.is_empty() {
        let (ex, remainder) = parse_expr(token_slice)?;
        log::trace!("parsed statement {:?}", ex);
        token_slice = remainder;
        expressions.push(ex);

        if !token_slice.is_empty() {
            token_slice = expect_token(Token::Semicolon, token_slice)?;
        }
    }

    Ok(AST { expressions })
}

fn parse_expr(tokens: &[Token]) -> Result<(Expression, &[Token]), String> {
    parse_function_call(tokens)
        .map(|(fc, ts)| (Expression::FunctionExpr(fc), ts))
        .or(parse_value(tokens).map(|(v, ts)| (Expression::ValueExpr(v), ts)))
}

// function-call := function-name '(' [ expr [ ',' expr ]* ]? ')'
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

fn expect_token(token: Token, tokens: &[Token]) -> Result<&[Token], String> {
    match tokens.get(0) {
        Some(t) if *t == token => Ok(&tokens[1..]),
        _ => Err(String::from("Expected char not found")),
    }
}

fn parse_value(tokens: &[Token]) -> Result<(Value, &[Token]), String> {
    parse_item(tokens)
        .or(parse_identifier(tokens).map(|(i, ts)| (Value::Identifier(i), ts)))
        .or(parse_number(tokens).map(|(n, ts)| (Value::Number(n), ts)))
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

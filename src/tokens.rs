#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Alphanum(String),
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Underscore,
    Semicolon,
    Comma,
    EqualSign,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alphanum(s) => f.write_str(s),
            Token::OpenParen => f.write_str("("),
            Token::CloseParen => f.write_str(")"),
            Token::OpenBracket => f.write_str("["),
            Token::CloseBracket => f.write_str("]"),
            Token::OpenBrace => f.write_str("{"),
            Token::CloseBrace => f.write_str("}"),
            Token::Underscore => f.write_str("_"),
            Token::Semicolon => f.write_str(";"),
            Token::Comma => f.write_str(","),
            Token::EqualSign => f.write_str("="),
        }
    }
}

enum State {
    Init,
    InAlphanum,
}

pub fn scan(source_code: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut charbuffer = vec![];
    let mut state = State::Init;
    for ch in source_code.chars() {
        match (&state, ch) {
            (State::Init, _) if ch.is_whitespace() => continue,
            (State::Init, _) if ch.is_alphanumeric() => {
                charbuffer.push(ch);
                state = State::InAlphanum;
            }
            (State::Init, '(') => tokens.push(Token::OpenParen),
            (State::Init, ')') => tokens.push(Token::CloseParen),
            (State::Init, '[') => tokens.push(Token::OpenBracket),
            (State::Init, ']') => tokens.push(Token::CloseBracket),
            (State::Init, '{') => tokens.push(Token::OpenBrace),
            (State::Init, '}') => tokens.push(Token::CloseBrace),
            (State::Init, ';') => tokens.push(Token::Semicolon),
            (State::Init, '_') => tokens.push(Token::Underscore),
            (State::Init, ',') => tokens.push(Token::Comma),
            (State::Init, '=') => tokens.push(Token::EqualSign),
            (State::Init, _) => return Result::Err(format!("Invalid character '{ch}'")),
            (State::InAlphanum, _) if ch.is_alphanumeric() || ch == '_' => {
                charbuffer.push(ch);
            }
            (State::InAlphanum, other) => {
                tokens.push(Token::Alphanum(charbuffer.iter().collect()));
                charbuffer.clear();

                match other {
                    ws if ws.is_whitespace() => {}
                    '(' => tokens.push(Token::OpenParen),
                    ')' => tokens.push(Token::CloseParen),
                    '[' => tokens.push(Token::OpenBracket),
                    ']' => tokens.push(Token::CloseBracket),
                    '{' => tokens.push(Token::OpenBrace),
                    '}' => tokens.push(Token::CloseBrace),
                    ';' => tokens.push(Token::Semicolon),
                    ',' => tokens.push(Token::Comma),
                    '=' => tokens.push(Token::EqualSign),
                    unexpected => return Result::Err(format!("Invalid character '{unexpected}'")),
                }

                state = State::Init;
            }
        }
    }
    if !charbuffer.is_empty() {
        let token = Token::Alphanum(charbuffer.iter().collect());
        tokens.push(token);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_statement() {
        let source_code = String::from("print(_[2]);");
        let expected = vec![
            Token::Alphanum(String::from("print")),
            Token::OpenParen,
            Token::Underscore,
            Token::OpenBracket,
            Token::Alphanum(String::from("2")),
            Token::CloseBracket,
            Token::CloseParen,
            Token::Semicolon,
        ];
        let actual = scan(&source_code).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_superfluous_ws() {
        let source_code = String::from("print   (_);");
        let expected = vec![
            Token::Alphanum(String::from("print")),
            Token::OpenParen,
            Token::Underscore,
            Token::CloseParen,
            Token::Semicolon,
        ];
        let actual = scan(&source_code).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_underscore_identifier() {
        let source_code = String::from("foo_bar");
        let expected = vec![Token::Alphanum(String::from("foo_bar"))];
        let actual = scan(&source_code).unwrap();
        assert_eq!(actual, expected);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Alphanum(String),
    OpenParen,
    CloseParen,
    Underscore,
    Semicolon,
}

enum State {
    Init,
    InAlphanum,
}

pub fn scan(source_code: String) -> Result<Vec<Token>, String> {
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
            (State::Init, ';') => tokens.push(Token::Semicolon),
            (State::Init, '_') => tokens.push(Token::Underscore),
            (State::Init, _) => return Result::Err(format!("Invalid character '{}'", ch)),
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
                    ';' => tokens.push(Token::Semicolon),
                    unexpected => {
                        return Result::Err(format!("Invalid character '{}'", unexpected))
                    }
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
        let source_code = String::from("print(_);");
        let expected = vec![
            Token::Alphanum(String::from("print")),
            Token::OpenParen,
            Token::Underscore,
            Token::CloseParen,
            Token::Semicolon,
        ];
        let actual = scan(source_code).unwrap();
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
        let actual = scan(source_code).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_underscore_identifier() {
        let source_code = String::from("foo_bar");
        let expected = vec![Token::Alphanum(String::from("foo_bar"))];
        let actual = scan(source_code).unwrap();
        assert_eq!(actual, expected);
    }
}

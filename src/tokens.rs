#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Alphanum(String),
    OpenParen,
    CloseParen,
    Underscore,
    Semicolon,
}

pub fn scan(source_code: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut current = vec![];
    for ch in source_code.chars() {
        let start_new_token: bool = current.is_empty()
            || ch.is_whitespace()
            || ch == '_' // TODO foo_bar is also a token
            || ch == '('
            || ch == ')'
            || ch == ';'
            || ch.is_alphanumeric() && !current.iter().all(|c: &char| c.is_alphanumeric());

        if start_new_token && !current.is_empty() {
            let token = to_token(&current)?;
            tokens.push(token);
            current.clear();
        }

        if !ch.is_whitespace() {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        let token = to_token(&current)?;
        tokens.push(token);
        current.clear();
    }
    Ok(tokens)
}

fn to_token(cs: &Vec<char>) -> Result<Token, String> {
    match cs[..] {
        ['_'] => Ok(Token::Underscore),
        [';'] => Ok(Token::Semicolon),
        ['('] => Ok(Token::OpenParen),
        [')'] => Ok(Token::CloseParen),
        _ if cs.iter().all(|c| c.is_alphanumeric())
            && cs.get(0).map(|c| c.is_alphabetic()).unwrap() =>
        {
            Ok(Token::Alphanum(cs.iter().collect()))
        }
        _ => Result::Err(format!("Invalid token '{}'", cs.iter().collect::<String>())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_statement() {
        let source_code = String::from("println   (_);");
        let expected = vec![
            Token::Alphanum(String::from("println")),
            Token::OpenParen,
            Token::Underscore,
            Token::CloseParen,
            Token::Semicolon,
        ];
        let actual = scan(source_code).unwrap();
        assert_eq!(actual, expected);
    }
}

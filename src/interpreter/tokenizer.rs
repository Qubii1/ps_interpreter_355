use super::value::Value;

#[derive(Clone, Debug)]
pub enum Token {
    Literal(Value),
    ExecName(String),
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    for raw in src.split_whitespace() {
        if let Ok(i) = raw.parse::<i32>() {
            tokens.push(Token::Literal(Value::Int(i)));
        } else if raw == "true" {
            tokens.push(Token::Literal(Value::Bool(true)));
        } else if raw == "false" {
            tokens.push(Token::Literal(Value::Bool(false)));
        } else if raw.starts_with('/') {
            tokens.push(Token::Literal(Value::Name(raw[1..].to_string())));
        } else {
            tokens.push(Token::ExecName(raw.to_string()));
        }
    }

    Ok(tokens)
}
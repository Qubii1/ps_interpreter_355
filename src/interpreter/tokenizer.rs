// -----------------------------------------------------------------------------
// File: tokenizer.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Defines a token enum and provides a tokenizer method for tokenizing a 
// given input string.
// -----------------------------------------------------------------------------

use super::value::Value;


#[derive(Clone, Debug)]

// Tokens are either literals which are handled in the
// Value enum, or function names which need to be executed.
pub enum Token
{
    // Any valid Value enum.
    Literal(Value),

    // Any function name to be executed.
    ExecName(String),
}

// Method will tokenize a string and return 
pub fn tokenize(src: &str) -> Result<Vec<Token>, String>
{
    // Create a vector that will hold the parsed tokens.
    let mut tokens = Vec::new();

    // Split the source string that we want to parse by the whitespaces
    // and for every token in that string 
    // assign it respectively to its correct Value.
    for raw in src.split_whitespace()
    {
        if let Ok(i) = raw.parse::<i32>()
        {
            // The token is an integer type.
            tokens.push(Token::Literal(Value::Int(i)));
        }
        else if raw == "true"
        {
            // The token is a boolean type with the value true.
            tokens.push(Token::Literal(Value::Bool(true)));
        }
        else if raw == "false"
        {
            // The token is a boolean type with the value false.
            tokens.push(Token::Literal(Value::Bool(false)));
        }
        else if raw.starts_with('/')
        {
            // The token is a variable type.
            tokens.push(Token::Literal(Value::Name(raw[1..].to_string())));
        }
        else
        {
            // The token is not a Value, it must be a defined variable or a function name.
            tokens.push(Token::ExecName(raw.to_string()));
        }
    }

    // Return the vector Result.
    Ok(tokens)
}
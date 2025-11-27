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

// Method will assign a token to its respective Value.
pub fn parse_atomic_token(raw: &str) -> Result<Token, String>
{
    if let Ok(i) = raw.parse::<i32>()
    {
        // The token is an integer type.
        return Ok(Token::Literal(Value::Int(i)));
    }

    if let Ok(f ) = raw.parse::<f64>()
    {
        return Ok(Token::Literal(Value::Real(f)));
    }
    if raw == "true"
    {
        // The token is a boolean type with the value true.
        return Ok(Token::Literal(Value::Bool(true)));
    }

    if raw == "false"
    {
        // The token is a boolean type with the value false.
        return Ok(Token::Literal(Value::Bool(false)));
    }

    if raw.starts_with('/')
    {
        // The token is a variable type.
        return Ok(Token::Literal(Value::Name(raw[1..].to_string())));
    }
    
    // The token is not a Value, it must be a defined variable or a function name.
    Ok(Token::ExecName(raw.to_string()))
    
}

// Method will tokenize a string and return 
pub fn tokenize(src: &str) -> Result<Vec<Token>, String>
{
    // Create a vector that will hold the parsed tokens.
    let mut tokens = Vec::new();

    // Convert to character iterator so we can parse { }, ( )
    let mut chars = src.chars().peekable();

    // Main tokenizer loop (replaces split_whitespace only)
    while let Some(&atomic_character) = chars.peek() {

        // Skip whitespace
        if atomic_character.is_whitespace() {
            chars.next();
            continue;
        }

        // Parse procedure literal { ... }
        if atomic_character == '{' 
        {
            // Consume '{'
            chars.next(); 

            // Body of the procedure
            let mut body = String::new();

            // Need to track depth because of nested procedures.
            let mut depth = 1; 

            while let Some(current_character) = chars.next() 
            {
                match current_character 
                {
                    '{' => 
                    { 
                        depth += 1; body.push(current_character); 
                    }

                    '}' => 
                    {
                        depth -= 1;
                        if depth == 0 
                        {
                            // Procedure literal finished
                            break; 
                        }
                        else
                        {
                            body.push(current_character);
                        }
                    }
                    _ => body.push(current_character)
                }
            }

            if depth != 0 
            {
                return Err("Unmatched '{' in procedure literal".into());
            }

            // Recursively tokenize the inside of the procedure
            let inner = tokenize(&body)?;
            tokens.push(Token::Literal(Value::Procedure(inner, None)));
            continue;
        }

        // String literal ( ... )
        if atomic_character == '(' 
        {
            // consume '('
            chars.next(); 
            let mut string_to_push = String::new();

            while let Some(current_character) = chars.next() 
            {
                if current_character == ')' 
                {
                    break;
                }
                string_to_push.push(current_character);
            }

            tokens.push(Token::Literal(Value::Str(string_to_push)));
            continue;
        }

        // Defualt will be to parse an atomic token (continuous non-whitespace characters)
        let mut raw = String::new();
        while let Some(&current_character) = chars.peek() 
        {
            if current_character.is_whitespace() || current_character == '{' || current_character == '}' || current_character == '(' || current_character == ')' 
            {
                break;
            }
            raw.push(current_character);
            chars.next();
        }

        if !raw.is_empty() 
        {
            tokens.push(parse_atomic_token(&raw)?);
        }
    }

    // Return the vector Result.
    Ok(tokens)
}
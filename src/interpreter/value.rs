// -----------------------------------------------------------------------------
// File: value.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Defines the valid values that can be pushed onto the interpreter
// operand stack.
// -----------------------------------------------------------------------------

use super::dict::EnvRef;

// These are the values that we want to push onto the stack
// when encountered.
#[derive(Clone, Debug)]
pub enum Value
{
    // Integer values
    Int(i32),

    // Floating point values
    Real(f64),

    // Boolean values
    Bool(bool),

    // Strings
    Str(String),
    
    // Variable names (ex. /x)
    Name(String),

    // Procedure blocks (ex. {5 2 add})
    Procedure(Vec<super::tokenizer::Token>, Option<EnvRef>),
}

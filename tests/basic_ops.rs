// -----------------------------------------------------------------------------
// File: basic_ops.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Unit tests for basic operations.
// -----------------------------------------------------------------------------

use ps_interpreter::{Interpreter, ScopeMode};
use ps_interpreter::interpreter::value::Value;

// Normal test case to ensure the add function is working properly.
#[test]
fn test_add()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("2 3 add").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(n) => assert_eq!(*n, 5),
        _ => panic!("expected int"),
    }
}

// Normal test case to ensure the def function is working properly.
#[test]
fn test_def()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("/x 10 def x").unwrap();
    
    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(10) => {}
        _ => panic!("expected 10"),
    }
}
// -----------------------------------------------------------------------------
// File: dictionary_operation_tests.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Unit tests for basic operations.
// -----------------------------------------------------------------------------

use ps_interpreter::{Interpreter, ScopeMode};
use ps_interpreter::interpreter::value::Value;


// Normal test case to ensure dict functionality is working
#[test]
fn test_dict_normal() {
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    postscript_interpreter.interpret("5 dict").unwrap();

    let stack = postscript_interpreter.opstack_snapshot();
    let top = stack.last().unwrap();

    match top 
    {
        Value::Dict(d) => assert!(d.is_empty(), "dict should create an empty dictionary"),
        _ => panic!("dict should push a dictionary"),
    }
}

// Edge case test to ensure if dict size is not an integer
// it should throw an error
#[test]
fn test_dict_type_error() {
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    let result = interp.interpret("(hello) dict");

    assert!(result.is_err(), "dict should fail if argument is not an integer");
}


// Normal test case to ensure the def function is working properly.
#[test]
fn test_def_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("/x 10 def x").unwrap();
    
    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(10) => {}
        _ => panic!("expected 10"),
    }
}

// Normal test case to ensure the clear function is working properly.
#[test]
fn test_clear_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("1 2 3 4").unwrap();

    let mut stack = postscript_interpreter.opstack_snapshot();

    assert_eq!(stack.len(), 4);

    match (&stack[0], &stack[1], &stack[2], &stack[3])
    {
        (Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4)) => {},
        _ => panic!("exch failed: expected [20, 10]"),
    }

    postscript_interpreter.interpret("clear").unwrap();

    stack = postscript_interpreter.opstack_snapshot();

    assert_eq!(stack.len(), 0);
}
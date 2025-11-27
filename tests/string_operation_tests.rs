// -----------------------------------------------------------------------------
// File: string_operation_tests.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Unit tests to ensure dynamic and lexical scoping works correctly and can
// be changed by the user.
// -----------------------------------------------------------------------------

use ps_interpreter::{Interpreter, ScopeMode};
use ps_interpreter::interpreter::value::Value;

// Normal test case for ensuring length functionality for strings 
// works correctly
#[test]
fn test_length_string_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    postscript_interpreter.interpret("(hello) length").unwrap();

    let top = postscript_interpreter.peek().unwrap();
    match top
    {
        Value::Int(n) => assert_eq!(*n, 5),
        _ => panic!("expected length 5"),
    }
}

// Edge test case to ensure if length is not called on a string
// it throws error
#[test]
fn test_length_string_invalid_type()
{
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    // 10 length → invalid (numbers don’t have length)
    let result = interp.interpret("10 length");

    assert!(result.is_err(), "length should fail on number");
}


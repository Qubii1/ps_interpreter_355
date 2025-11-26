// -----------------------------------------------------------------------------
// File: scoping.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Unit tests to ensure dynamic and lexical scoping works correctly and can
// be changed by the user.
// -----------------------------------------------------------------------------

use ps_interpreter::{Interpreter, ScopeMode};
use ps_interpreter::interpreter::value::Value;

// Normal test case to ensure dynamic scoping is working.
#[test]
fn test_dynamic_scope()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    // /x 10 def
    // { x }  pushing the value should see dynamic x
    postscript_interpreter.interpret("/x 10 def x").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(10) => {}
        _ => panic!("expected 10"),
    }
}

// Normal test case for ensuring lexical scoping is working.
#[test]
fn test_lexical_scope_basic() {
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Lexical);

    // Define x = 10
    postscript_interpreter.interpret("/x 10 def").unwrap();

    // Define f as a procedure that returns x
    postscript_interpreter.interpret("/f { x } def").unwrap();

    // Shadow x at runtime
    postscript_interpreter.interpret("/x 20 def").unwrap();

    // Call f
    postscript_interpreter.interpret("f").unwrap();

    // Lexical scope → should use x = 10
    match postscript_interpreter.peek().unwrap() {
        Value::Int(10) => {},
        _ => panic!("Lexical scoping failed: expected 10"),
    }
}

// Normal test case for testing nested lexical scoping.
#[test]
fn test_lexical_scope_nested() {
    let mut interp = Interpreter::new(ScopeMode::Lexical);

    // Outer variable
    interp.interpret("/a 100 def").unwrap();

    // g returns a
    interp.interpret("/g { a } def").unwrap();

    // f redefines a, then calls g
    // Under lexical scoping:
    // - g should see the a from where *g was defined*, not the a inside f
    interp.interpret("/f { /a 200 def g } def").unwrap();

    // Run f → should call g → should see a = 100
    interp.interpret("f").unwrap();

    match interp.peek().unwrap() {
        Value::Int(100) => {},
        _ => panic!("Lexical scoping failed: nested lookup should use original 'a' = 100"),
    }
}

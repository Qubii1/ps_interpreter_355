// -----------------------------------------------------------------------------
// File: basic_ops.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Unit tests to ensure dynamic and lexical scoping works correctly and can
// be changed by the user.
// -----------------------------------------------------------------------------

use ps_interpreter::{Interpreter, ScopeMode};
use ps_interpreter::interpreter::value::Value;

#[test]
fn test_dynamic_scope()
{
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    // /x 10 def
    // { x }  pushing the value should see dynamic x
    interp.interpret("/x 10 def x").unwrap();

    match interp.peek().unwrap()
    {
        Value::Int(10) => {}
        _ => panic!("expected 10"),
    }
}
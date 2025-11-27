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
fn test_add_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("2 3 add").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(n) => assert_eq!(*n, 5),
        _ => panic!("expected int"),
    }
}

// Normal test case to ensure the mul function is working properly.
#[test]
fn test_mul_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("2 3 mul").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(n) => assert_eq!(*n, 6),
        _ => panic!("expected int"),
    }
}

// Normal test case to ensure the sub function is working properly.
#[test]
fn test_sub_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("3 2 sub").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(n) => assert_eq!(*n, 1),
        _ => panic!("expected int"),
    }
}

// Normal test case to ensure the div function is working properly.
#[test]
fn test_div_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("6 3 div").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(n) => assert_eq!(*n, 2),
        _ => panic!("expected int"),
    }
}

// Normal test case to ensure the div function is working properly.
#[test]
fn test_mod_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("6 3 mod").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(n) => assert_eq!(*n, 0),
        _ => panic!("expected int"),
    }
}

// Normal test case to ensure the dup function is working properly.
#[test]
fn test_dup_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
    postscript_interpreter.interpret("1").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(n) => assert_eq!(*n, 1),
        _ => panic!("expected int"),
    }

    postscript_interpreter.interpret("dup").unwrap();

    match postscript_interpreter.peek().unwrap()
    {
        Value::Int(n) => assert_eq!(*n, 1),
        _ => panic!("expected int"),
    }
}

// Edge case test to ensure that dup on empty stack results in error
#[test]
fn test_dup_underflow() {
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    let result = interp.interpret("dup");

    assert!(result.is_err(), "dup on an empty stack should produce an error");
}

// Normal test case to ensure copy function is working properly
#[test]
fn test_copy_normal() 
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    // 1 2 3 2 copy
    postscript_interpreter.interpret("1 2 3 2 copy").unwrap();

    let stack = postscript_interpreter.opstack_snapshot();

    // Expected: [1, 2, 3, 2, 3]
    assert_eq!(stack.len(), 5);

    match (&stack[0], &stack[1], &stack[2], &stack[3], &stack[4]) 
    {
        (Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(2), Value::Int(3)) => {}
        _ => panic!("copy did not duplicate correctly"),
    }
}


// Normal test case to ensure pop function is working properly
#[test]
fn test_pop_normal() {
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    interp.interpret("1 2 3").unwrap(); // stack: [1, 2, 3]
    interp.interpret("pop").unwrap();   // remove 3

    match interp.peek().unwrap()
    {
        Value::Int(2) => {},
        _ => panic!("Expected top of stack to be 2 after pop"),
    }
}

// Edge test case for ensuring when stack is empty popping will return correct error.
#[test]
fn test_pop_underflow() {
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    let result = interp.interpret("pop");

    assert!(result.is_err(), "pop on empty stack should return an error");
}

// Normal test case to ensure exch function is working properly.
#[test]
fn test_exch_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    postscript_interpreter.interpret("10 20 exch").unwrap(); // should become [20, 10]

    let stack = postscript_interpreter.opstack_snapshot();

    assert_eq!(stack.len(), 2);

    match (&stack[0], &stack[1])
    {
        (Value::Int(20), Value::Int(10)) => {},
        _ => panic!("exch failed: expected [20, 10]"),
    }
}

// Edge test case to ensure exch function results in error if there
// is less than two values on the stack.
#[test]
fn test_exch_underflow()
{
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    let result = interp.interpret("1 exch");
    assert!(result.is_err(), "exch with one value should error");

    let result2 = interp.interpret("exch");
    assert!(result2.is_err(), "exch with zero values should error");
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
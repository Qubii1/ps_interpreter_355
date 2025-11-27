// -----------------------------------------------------------------------------
// File: arithmetic_operation_tests.rs
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
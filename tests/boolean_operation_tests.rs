// -----------------------------------------------------------------------------
// File: boolean_operation_tests.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Unit tests for basic operations.
// -----------------------------------------------------------------------------

use ps_interpreter::{Interpreter, ScopeMode};
use ps_interpreter::interpreter::value::Value;

// Normal test case for ensuring eq functionality works correctly
#[test]
fn test_eq_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("5 5 eq").unwrap();

    match i.peek().unwrap()
    {
        Value::Bool(true) => {},
        _ => panic!("expected true"),
    }
}

// Edge test case for ensuring eq throws error on bad types
#[test]
fn test_eq_type_error()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);
    let result = i.interpret("5 true eq");
    assert!(result.is_err());
}

// Normal test case for ensuring ne functionality works correctly
#[test]
fn test_ne_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("4 5 ne").unwrap();

    match i.peek().unwrap()
    {
        Value::Bool(true) => {},
        _ => panic!("expected true"),
    }
}

// Normal test case for ensuring gt functionality works correctly
#[test]
fn test_gt_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("10 2 gt").unwrap();

    match i.peek().unwrap()
    {
        Value::Bool(true) => {},
        _ => panic!("expected true"),
    }
}

// Edge test case for ensuring gt throws error on bad types
#[test]
fn test_gt_type_error()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);
    let result = i.interpret("true 5 gt");
    assert!(result.is_err());
}

// Normal test case for ensuring lt functionality works correctly
#[test]
fn test_lt_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("1 3 lt").unwrap();

    match i.peek().unwrap()
    {
        Value::Bool(true) => {},
        _ => panic!("expected true"),
    }
}

// Normal test case for ensuring and functionality works correctly
#[test]
fn test_and_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("true true and").unwrap();

    match i.peek().unwrap()
    {
        Value::Bool(true) => {},
        _ => panic!("expected true"),
    }
}

// Edge test case for ensuring and throws error on bad types
#[test]
fn test_and_type_error()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);
    let result = i.interpret("5 true and");
    assert!(result.is_err());
}

// Normal test case for ensuring or functionality works correctly
#[test]
fn test_or_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("false true or").unwrap();

    match i.peek().unwrap()
    {
        Value::Bool(true) => {},
        _ => panic!("expected true"),
    }
}

// Normal test case for ensuring not functionality works correctly
#[test]
fn test_not_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("true not").unwrap();

    match i.peek().unwrap()
    {
        Value::Bool(false) => {},
        _ => panic!("expected false"),
    }
}

// Edge test case for ensuring not throws error on bad types
#[test]
fn test_not_type_error()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);
    let result = i.interpret("5 not");
    assert!(result.is_err());
}
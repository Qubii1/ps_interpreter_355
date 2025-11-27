// -----------------------------------------------------------------------------
// File: flow_control_operation_tests.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Unit tests for basic operations.
// -----------------------------------------------------------------------------

use ps_interpreter::{Interpreter, ScopeMode};
use ps_interpreter::interpreter::value::Value;

// Normal test case to ensure if functionality is working correctly
#[test]
fn test_if_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("true { 7 } if").unwrap();

    match i.peek().unwrap()
    {
        Value::Int(7) => {},
        _ => panic!("expected 7"),
    }
}

// Edge test case to ensure if procedure is invalid
// if throws error
#[test]
fn test_if_type_error()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);
    let res = i.interpret("5 { 1 } if");
    assert!(res.is_err());
}

// Normal test case to ensure ifelse true branch functionality is working correctly
#[test]
fn test_ifelse_true_branch() 
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("true { 1 } { 2 } ifelse").unwrap();

    match i.peek().unwrap()
    {
        Value::Int(1) => {},
        _ => panic!("expected 1"),
    }
}

// Normal test case to ensure ifelse false branch functionality is working correctly
#[test]
fn test_ifelse_false_branch() 
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("false { 1 } { 2 } ifelse").unwrap();

    match i.peek().unwrap()
    {
        Value::Int(2) => {},
        _ => panic!("expected 2"),
    }
}

// Normal test case to ensure repeat functionality is working correctly

#[test]
fn test_repeat()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("3 { 10 } repeat").unwrap();

    assert_eq!(i.len(), 3);
}

// Normal test case to ensure for functionality is working correctly
#[test]
fn test_for_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("1 1 3 { } for").unwrap();

    assert_eq!(i.len(), 3); // loop ran 3 times
}

// Normal test case to ensure for functionality is working correctly
#[test]
fn test_for_values()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    i.interpret("1 1 3 { } for").unwrap();

    let stack = i.opstack_snapshot();
    assert_eq!(stack.len(), 3);

    match &stack[0]
    {
        Value::Int(1) => {}
        _ => panic!("expected 1"),
    }

    match &stack[1]
    {
        Value::Int(2) => {}
        _ => panic!("expected 2"),
    }

    match &stack[2]
    {
        Value::Int(3) => {}
        _ => panic!("expected 3"),
    }

}


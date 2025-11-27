// -----------------------------------------------------------------------------
// File: boolean_operation_tests.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Unit tests for basic operations.
// -----------------------------------------------------------------------------

use ps_interpreter::{Interpreter, ScopeMode};

// Normal test case to ensure print functionality works correctly
#[test]
fn test_print_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    // Should not panic
    let res = i.interpret("(hello) print");
    assert!(res.is_ok());

    // Stack should now be empty
    assert_eq!(i.len(), 0);
}

// Edge test case to throw error on bad print types
#[test]
fn test_print_type_error()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    let result = i.interpret("5 print");
    assert!(result.is_err());
}

// Normal test case to ensure = functionality works correctly
#[test]
fn test_equals_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    let res = i.interpret("42 =");
    assert!(res.is_ok());

    assert_eq!(i.len(), 0);
}

// Normal test case to ensure == functionality works correctly
#[test]
fn test_literal_equals_normal()
{
    let mut i = Interpreter::new(ScopeMode::Dynamic);

    let res = i.interpret("(hello) ==");
    assert!(res.is_ok());

    assert_eq!(i.len(), 0);
}


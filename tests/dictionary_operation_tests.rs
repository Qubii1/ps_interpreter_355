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
fn test_dict_normal() 
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    postscript_interpreter.interpret("5 dict").unwrap();

    let stack = postscript_interpreter.opstack_snapshot();
    let top = stack.last().unwrap();

    match top 
    {
        Value::Dict(dictionary) => assert!(dictionary.borrow().is_empty(), "dict should create an empty dictionary"),
        _ => panic!("dict should push a dictionary"),
    }
}

// Edge case test to ensure if dict size is not an integer
// it should throw an error
#[test]
fn test_dict_type_error() 
{
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

// Normal test case to ensure begin functionality is working
#[test]
fn test_begin_normal() 
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    // Push a dictionary onto the operand stack, then begin it
    postscript_interpreter.interpret("5 dict begin").unwrap();

    // Confirm dictionary stack grew
    let env = postscript_interpreter.dict.env();
    let borrowed = env.borrow();

    assert_eq!(borrowed.len(), 2, "begin should push a new dictionary frame");
}

// Edge test case for ensuring if begin doesn't access a dict value it 
// throws an error
#[test]
fn test_begin_type_error() 
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    // Try to begin with an integer on stack
    let result = postscript_interpreter.interpret("10 begin");

    assert!(result.is_err(), "begin must error if top of stack is not a dictionary");
}

// Normal test case to ensure end functionality is working
#[test]
fn test_end_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    // Create a new dictionary and begin it
    postscript_interpreter.interpret("5 dict begin").unwrap();

    // Dictionary stack should now have 2 frames
    assert_eq!(postscript_interpreter.dict.env().borrow().len(), 2);

    // Now end the scope
    postscript_interpreter.interpret("end").unwrap();

    // Back to only 1 frame
    assert_eq!(postscript_interpreter.dict.env().borrow().len(), 1);
}

// Edge test case to ensure that if the dictionary stack
// only has one dictionary it should throw error if you try to end it
#[test]
fn test_end_underflow()
{
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    // Try to end with only bottom dict in stack
    let result = interp.interpret("end");

    assert!(result.is_err(), "end should error when popping bottom dictionary");
}

// Normal test case to ensure lenght for dictionary is working
#[test]
fn test_length_dictionary_normal()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);
 
    postscript_interpreter.interpret("2 dict").unwrap();      // create dict with capacity 2
    postscript_interpreter.interpret("dup begin").unwrap();   // begin scope on the same dict (dup keeps it)
    postscript_interpreter.interpret("/x 10 def /y 20 def").unwrap(); // define some variables
    //postscript_interpreter.interpret("end").unwrap();         // leave dict on operand stack
    postscript_interpreter.interpret("length").unwrap();      // ask for dictionary length

    let stack = postscript_interpreter.opstack_snapshot();
    let top = stack.last().unwrap();

    match top
    {
        Value::Int(n) => assert_eq!(*n, 2),
        _ => panic!("expected dictionary length = 2"),
    }
}


// Edge case test to ensure if dictionary is empty it doesnt throw error
// should just return 0
#[test]
fn test_length_empty_dictionary()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    postscript_interpreter.interpret("0 dict length").unwrap();

    let top = postscript_interpreter.peek().unwrap();

    match top
    {
        Value::Int(n) => assert_eq!(*n, 0),
        _ => panic!("expected dictionary length = 0"),
    }
}




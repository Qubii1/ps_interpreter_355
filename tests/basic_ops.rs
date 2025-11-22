use ps_interpreter::{Interpreter, ScopeMode};
use ps_interpreter::interpreter::value::Value;

#[test]
fn test_add() {
    let mut interp = Interpreter::new(ScopeMode::Dynamic);
    interp.interpret("2 3 add").unwrap();

    match interp.peek().unwrap() {
        Value::Int(n) => assert_eq!(*n, 5),
        _ => panic!("expected int"),
    }
}

#[test]
fn test_def() {
    let mut interp = Interpreter::new(ScopeMode::Dynamic);
    interp.interpret("/x 10 def x").unwrap();
    
    match interp.peek().unwrap() {
        Value::Int(10) => {}
        _ => panic!("expected 10"),
    }
}
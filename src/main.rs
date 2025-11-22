use ps_interpreter::{Interpreter, ScopeMode};

fn main() {
    let mut interp = Interpreter::new(ScopeMode::Dynamic);

    let program = "/x 10 def x 5 add";
    println!("Running: {}", program);

    match interp.interpret(program) {
        Ok(()) => println!("Final stack: {:?}", interp.opstack_snapshot()),
        Err(e) => println!("Error: {}", e),
    }
}
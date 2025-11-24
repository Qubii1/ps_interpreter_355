use ps_interpreter::{Interpreter, ScopeMode};

fn main()
{
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    let program = "/x 10 def x 5 add";
    println!("Running: {}", program);

    match postscript_interpreter.interpret(program)
    {
        Ok(()) => println!("Final stack: {:?}", postscript_interpreter.opstack_snapshot()),
        Err(e) => println!("Error: {}", e),
    }
}
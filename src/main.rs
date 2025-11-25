// -----------------------------------------------------------------------------
// File: main.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// The entry point to the Rust postscript interpreter project.
// -----------------------------------------------------------------------------


use ps_interpreter::{Interpreter, ScopeMode};

fn main()
{
    // Create an interpreter.
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Dynamic);

    // Give the program an input string to execute.
    let program = "/x 10 def x 5 add";
    println!("Running: {}", program);

    // Run the interpreter.
    match postscript_interpreter.interpret(program)
    {
        Ok(()) => println!("Final stack: {:?}", postscript_interpreter.opstack_snapshot()),
        Err(e) => println!("Error: {}", e),
    }
}
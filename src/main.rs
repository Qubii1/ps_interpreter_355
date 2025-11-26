// -----------------------------------------------------------------------------
// File: main.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// The entry point to the Rust postscript interpreter project.
// -----------------------------------------------------------------------------

use std::io::{self, Write};
use ps_interpreter::{Interpreter, ScopeMode};

fn main()
{
    // Create an interpreter.
    let mut postscript_interpreter = Interpreter::new(ScopeMode::Lexical);

    // Give the program an input string to execute.
    //let program = "/x 10 def x 5 add";
    //println!("Running: {}", program);

    // REPL
    loop 
    {
        // print prompt like it is in postscript, and flush so it appears immediately.
        // without the flush Rust doesnt actually print it to the console.
        if postscript_interpreter.len() > 0
        {
            // something is in the stack so show the size
            print!("PS<{}> ", postscript_interpreter.len());
        }
        else
        {
            // nothing is in the stack just print prompt
            print!("PS> ");
        }
        io::stdout().flush().unwrap();

        // read the input
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // trim the whitespace
        let trimmed = input.trim();

        // if user enters quit the REPL will stop
        // this ignore case just makes it so the user
        // doesnt have to worry about uppercase or lowercase
        if trimmed.eq_ignore_ascii_case("quit")
        {
            println!("Exiting interpreter...");
            break;
        }

        // ignores empty input
        if trimmed.is_empty()
        {
            continue;
        }

        // run the interpreter.
        match postscript_interpreter.interpret(trimmed)
        {
            Ok(()) => 
            {
                println!("Final stack: {:?}", postscript_interpreter.opstack_snapshot());
            }
            Err(e) => 
            {
                println!("Error: {}", e);
            }
        }
    }
}
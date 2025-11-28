# Postscript interpreter in Rust
This project is a simplified PostScript interpreter written in Rust for CptS 355.
It supports arithmetic operations, boolean logic, string manipulation, dictionaries, procedures, flow control, and both dynamic and lexical scoping.

# Building the interpreter
This project uses Cargo which is a Rust package manager

-- Pre-requisites: 
To build the interpreter on your local machine
you first need to install the Rust language. This will also 
install the Cargo package manager to be able to run
the interpreter and run automated tests as well

To build the interpreter run this command in the terminal:

cargo build

To build and run the interpreter run this command in the terminal:

cargo run

To run all the tests in the project run this command in the terminal:

cargo test

# Running the interpreter

Once you enter cargo run in the terminal, the interpreter
will by default be in dynamic scoping mode. You can then interact
with the interpreter just as you would interact with something like
ghostscript. 

# Scoping flags

This interpreter supports both dynamic scoping by default since
postscript does use dynamic scoping,
and lexical scoping by adding a --lexical flag after running
cargo run. 

-- Dynamic scoping (default run)

Matches real PostScript. A name lookup starts from the topmost dictionary and searches downward until a match is found.

To build and run the interpreter with dynamic scoping simply run this command:

cargo run

-- Lexical scoping (Needs flag)

Captured at definition time. Procedures will 
remember the dictionary environment they were defined in

To build and run the interpreter with lexical scoping run this command:

cargo run -- --lexical

# How scoping works internally

Dynamic scoping will perform a live dictionary-stack lookup (lookup_dynamic)

Lexical scoping deep-copies the dictionary stack at definition time
and the procedures store this environment so
later procedure calls use lookup_lexical


// -----------------------------------------------------------------------------
// File: exec.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Contains the core interpreter execution logic, including token execution,
// name resolution, scoping rules (dynamic & lexical), and procedure calls.
// -----------------------------------------------------------------------------

use super::stack::OperandStack;
use super::dict::{DictStack, EnvRef};
use super::tokenizer::{Token, tokenize};
use super::value::Value;
use super::scope::ScopeMode;

pub type InterpreterResult = Result<(), String>;

// Defines the interpreter structure.
pub struct Interpreter
{
    // The operand stack that the interpreter uses for valid Values.
    pub opstack: OperandStack,

    // The dictionary stack that the interpreter uses.
    pub dict: DictStack,

    // The type of scoping the interpreter uses.
    pub scope_mode: ScopeMode,
}

impl Interpreter
{
    // Interpreter constructor.
    pub fn new(scope: ScopeMode) -> Self 
    {
        Self
        {
            opstack: OperandStack::new(),
            dict: DictStack::new(),
            scope_mode: scope,
        }
    }

    // Tokenizes the input and executes those tokens.
    pub fn interpret(&mut self, src: &str) -> InterpreterResult 
    {
        let tokens = tokenize(src)?;
        self.exec_tokens(&tokens, None)
    }

    // Executes the given input.
    pub fn exec_tokens(&mut self, tokens: &[Token], defining_env: Option<EnvRef>) -> InterpreterResult
    {
        for token in tokens
        {
            match token
            {
                // If token is a number, variable name, boolean value, string, or procedure body
                // simply push it onto the stack. 
                Token::Literal(v) => self.opstack.push(v.clone()),

                // Otherwise the token is an executable and needs to be resolved.
                Token::ExecName(name) =>
                {
                    if self.try_builtin(name)?
                    {
                        // Executed the token, move to the next one.
                        continue;
                    }

                    let resolved = match self.scope_mode
                    {
                        // Lookup for value in dict dynamically.
                        ScopeMode::Dynamic => self.dict.lookup_dynamic(name),

                        // Lookup for value in dict lexically.
                        ScopeMode::Lexical =>
                        {
                            if let Some(env) = &defining_env
                            {
                                self.dict.lookup_lexical(name, env)
                            }
                            else
                            {
                                self.dict.lookup_dynamic(name)
                            }
                        }
                    }.ok_or_else(|| format!("Undefined name: {}", name))?;

                    match resolved
                    {
                        Value::Procedure(body, captured_env) =>
                        {
                            // Value is a procedure that needs to be executed.
                            // Recursively call the function using the procedure body.
                            self.exec_tokens(&body, captured_env)?;
                        }

                        // Otherwise push the value to the stack if its
                        // not a procedure.
                        _ => self.opstack.push(resolved),
                    }
                }
            }
        }
        Ok(())
    }

    // These methods are for convenience when executing.
    pub fn push(&mut self, v: Value)
    {
        self.opstack.push(v);
    }

    pub fn pop(&mut self) -> Result<Value, String>
    { 
        self.opstack.pop()
    }
    pub fn peek(&self) -> Option<&Value>
    {
        self.opstack.peek()
    }
    pub fn opstack_snapshot(&self) -> Vec<Value>
    {
        self.opstack.snapshot()
    }
}
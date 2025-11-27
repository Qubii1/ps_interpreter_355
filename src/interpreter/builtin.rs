// -----------------------------------------------------------------------------
// File: builtin.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Defines the different builtin methods that the interpreter should be
// able to execute.
// -----------------------------------------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

use super::scope::ScopeMode;
use super::value::Value;
use super::exec::Interpreter;

impl Interpreter
{
    // Method that runs built-in executions. 
    pub fn try_builtin(&mut self, name: &str) -> Result<bool, String>
    {
        match name
        {
            // Adds two valid integer values.
            "add" =>
            {
                // Pop the first value off the stack.
                let b = self.pop()?;

                // Pop the second value off the stack.
                let a = self.pop()?;

                // Push the result of adding a + b to the stack.
                self.push(match (a, b)
                {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                    _ => return Err("Type error in add".to_string()),
                });

                Ok(true)
            }

            // Subtracts two valid integer values.
            "sub" =>
            {
                // Pop the first value off the stack.
                let b = self.pop()?;

                // Pop the second value off the top of stack.
                let a = self.pop()?;

                // Push the result of subtracting a - b to the stack.
                self.push(match (a, b)
                {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                    _ => return Err("Type error in sub".to_string()),
                });

                Ok(true)
            }

            // Duplicates the top of the Operand Stack
            "dup" =>
            {
                let top = self.peek().ok_or("Operand stack underflow")?.clone();

                self.push(top);

                Ok(true)
            }

            // Swaps the top two Values on the stack.
            "exch" =>
            {
                let b = self.pop()?;

                let a = self.pop()?;

                self.push(b);

                self.push(a);

                Ok(true)
            }

            // Pops the top of the stack.
            "pop" =>
            {
                self.pop()?;

                Ok(true)
            }

            "def" =>
            {
                // First get the value of the variable.
                let value = self.pop()?;

                // Then get the name associated with that value.
                let name = self.pop()?;

                if let Value::Name(n) = name
                {
                    match (&self.scope_mode, &value) 
                    {
                        // LEXICAL: capture the environment at definition time
                        ( ScopeMode::Lexical, Value::Procedure(body, _) ) =>
                        {
                            // Clone dictionary stack (Vec<Dict>) at definition time
                            let snapshot = self.dict.env().borrow().clone();

                            let captured = Value::Procedure(
                                body.clone(),
                                Some(Rc::new(RefCell::new(snapshot)))
                            );

                            self.dict.define(&n, captured);
                        }
                        // DYNAMIC: store the procedure as-is (no captured env)
                        _ =>
                        {
                            println!("DEBUG DEF: defining '{}' as {:?}", n, value);

                            self.dict.define(&n, value);
                        }
                    }

                    Ok(true)
                }
                else
                {
                    // Variable name was invalid.
                    Err("def expects a literal name".to_string())
                }
            }

            // Clears all the values in the stack
            "clear" =>
            {
                self.clear();

                Ok(true)
            }

            _ => Ok(false),
        }
    }
}

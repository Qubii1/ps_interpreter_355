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

            // Multiplies two valid integer values.
            "mul" =>
            {
                // Pop the first value off the stack.
                let b = self.pop()?;

                // Pop the second value off the stack.
                let a = self.pop()?;

                // Push the result of adding a * b to the stack.
                self.push(match (a, b)
                {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                    _ => return Err("Type error in add".to_string()),
                });

                Ok(true)
            }

            // Divides two valid integer values.
            "div" =>
            {
                // Pop the first value off the stack.
                let b = self.pop()?;

                // Pop the second value off the top of stack.
                let a = self.pop()?;

                // Push the result of subtracting a / b to the stack.
                self.push(match (a, b)
                {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x / y),
                    _ => return Err("Type error in sub".to_string()),
                });

                Ok(true)
            }

            // Mod two valid integer values.
            "mod" =>
            {
                // Pop the first value off the stack.
                let b = self.pop()?;

                // Pop the second value off the top of stack.
                let a = self.pop()?;

                // Push the result of subtracting a % b to the stack.
                self.push(match (a, b)
                {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x % y),
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

            // Copies the top n values and pushes them to the stack in order
            "copy" =>
            {
                // Pop the count
                let count_val = self.pop()?;

                // Extract an integer from the Value
                let n = match count_val 
                {
                    Value::Int(i) => i as usize,
                    _ => return Err("copy expects an integer count".into()),
                };

                // Ensure the stack has enough values
                let stack_len = self.opstack.len();
                if n > stack_len 
                {
                    return Err("copy expects more operands on the stack".into());
                }

                // Snapshot current stack
                let snapshot = self.opstack.snapshot();

                // Get the last n elements
                let slice = &snapshot[stack_len - n..];

                // Push clones back on stack
                for v in slice 
                {
                    self.push(v.clone());
                }

                Ok(true)
            }

            // Pops the top of the stack.
            "pop" =>
            {
                self.pop()?;

                Ok(true)
            }

            // gets the number of items on the operand stack
            "count" =>
            {
                // length of the operand stack
                let n = self.opstack.len() as i32;

                // push the count
                self.push(Value::Int(n));

                Ok(true)
            }

            "dict" =>
            {
                // Pop the size argument
                let size_val = self.pop()?;

                // Check that it's an int
                let size = match size_val 
                {
                    Value::Int(i) => i,
                    _ => return Err("dict expects an integer size".into()),
                };

                if size < 0 
                {
                    return Err("dict size cannot be negative".into());
                }

                // Create a new empty dictionary
                let new_dict = std::collections::HashMap::new();

                // Wrap it as Value::Dict
                self.push(Value::Dict(new_dict));

                Ok(true)
            }

            "begin" =>
            {
                // Pop value from operand stack
                let dict_val = self.pop()?;

                // Ensure it is a dictionary
                let new_dict = match dict_val 
                {
                    Value::Dict(d) => d,
                    _ => return Err("begin expects a dictionary".into()),
                };

                // Push dictionary onto dictionary stack
                let env_ref = self.dict.env(); // persistent dicitonary stack reference
                let mut env: std::cell::RefMut<'_, Vec<std::collections::HashMap<String, Value>>> = env_ref.borrow_mut(); // borrow 
                env.push(new_dict);

                Ok(true)
            }

            "end" =>
            {
                // Borrow the environment stack safely
                let env_ref = self.dict.env();
                let mut env = env_ref.borrow_mut();

                // You must not remove the bottom dictionary
                if env.len() <= 1 
                {
                    return Err("end cannot pop bottom dictionary".into());
                }

                env.pop();
                
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

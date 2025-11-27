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

use super::dict::Dict;
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
                let new_dict = Rc::new(RefCell::new(std::collections::HashMap::new()));

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
                let mut env  = env_ref.borrow_mut(); // borrow 
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

            "length" =>
            {
                let object = self.pop()?;

                let len = match object 
                {
                    Value::Str(string) => string.len() as i32,

                    Value::Dict(dictionary) => dictionary.borrow().len() as i32,

                    _ => return Err("length expects string, dict".into()),
                };

                self.push(Value::Int(len));
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
                             // Deep-copy the whole dictionary stack into a new EnvRef,
                            // but make sure the borrow does NOT live past this inner block.
                            let snapshot_vec: Vec<Dict> =
                            {
                                // Get current environment (vector of Dict = Rc<RefCell<HashMap<...>>>)
                                let env_ref = self.dict.env();
                                let env_borrow = env_ref.borrow();

                                let mut snapshot = Vec::new();
                                for dict_ref in env_borrow.iter()
                                {
                                    // &HashMap<String, Value>
                                    let dict = dict_ref.borrow();

                                    // deep copy the map
                                    let cloned_map = dict.clone();

                                    // wrap in a new Rc<RefCell<_>>
                                    let new_dict: Dict = Rc::new(RefCell::new(cloned_map));
                                    snapshot.push(new_dict);
                                }

                                snapshot   // <- returned out of the block
                            }; // <- env_borrow and env_ref are dropped RIGHT HERE

                            // Wrap the cloned vec in a new EnvRef
                            let captured_env = Rc::new(RefCell::new(snapshot_vec));

                            // Store procedure with its frozen lexical environment
                            let captured = Value::Procedure(body.clone(), Some(captured_env));
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

            "get" =>
            {
                // Pop index
                let index_val = self.pop()?;

                // Make sure its a valid integer value
                let index = match index_val
                {
                    Value::Int(i) => i,
                    _ => return Err("get expects integer index".into()),
                };

                // Invalid index should throw error
                if index < 0
                {
                    return Err("get index out of bounds".into());
                }

                // Pop string
                let string_val = self.pop()?;

                // Make sure string is valid
                let string = match string_val
                {
                    Value::Str(string_to_validate) => string_to_validate,
                    _ => return Err("get expects string".into()),
                };

                let i = index as usize;

                // Index greater than string length should throw error
                if i >= string.len()
                {
                    return Err("get index out of bounds".into());
                }

                // Convert char to i32 ASCII code
                let target_character = string.as_bytes()[i] as i32;

                // Push to stack
                self.push(Value::Int(target_character));
                Ok(true)
            }

            "getinterval" =>
            {
                // Pop count
                let count_val = self.pop()?;

                // Make sure count is a valid integer
                let count = match count_val
                {
                    Value::Int(i) => i,
                    _ => return Err("getinterval expects integer count".into()),
                };

                // Throws error if count is negative
                if count < 0
                {
                    return Err("getinterval count cannot be negative".into());
                }

                // Pop index
                let index_val = self.pop()?;

                // Make sure index is a valid integer
                let index = match index_val
                {
                    Value::Int(i) => i,
                    _ => return Err("getinterval expects integer index".into()),
                };

                // Throws error if index is negative
                if index < 0
                {
                    return Err("getinterval index cannot be negative".into());
                }

                // Pop string
                let string_val = self.pop()?;

                // Makes sure string is valid
                let string = match string_val
                {
                    Value::Str(string_to_check) => string_to_check,
                    _ => return Err("getinterval expects string".into()),
                };

                let string_index = index as usize;
                let character_count = count as usize;

                // Bounds check
                if string_index + character_count > string.len()
                {
                    return Err("getinterval range error".into());
                }

                // Extract substring
                let substring = string[string_index..string_index + character_count].to_string();

                // Push new string result
                self.push(Value::Str(substring));

                Ok(true)
            }

            "putinterval" =>
            {
                // Pop source string
                let source_val = self.pop()?;

                // Ensure string is valid
                let source = match source_val
                {
                    Value::Str(string_to_check) => string_to_check,
                    _ => return Err("putinterval expects source string".into()),
                };

                // Pop index
                let index_val = self.pop()?;

                // Ensure index is valid integer
                let index = match index_val
                {
                    Value::Int(i) => i,
                    _ => return Err("putinterval expects integer index".into()),
                };

                // Negative index should throw error
                if index < 0
                {
                    return Err("putinterval index cannot be negative".into());
                }

                let string_index = index as usize;

                // Pop target string
                let target_val = self.pop()?;

                // Ensure string is valid
                let mut target = match target_val
                {
                    Value::Str(string_to_check) => string_to_check,
                    _ => return Err("putinterval expects target string".into()),
                };

                // Bounds check
                if string_index + source.len() > target.len()
                {
                    return Err("putinterval range error".into());
                }

                // Mutate the target string
                for (i, current_character) in source.chars().enumerate()
                {
                    target.replace_range(string_index + i .. string_index + i + 1, &current_character.to_string());
                }

                // Push mutated string back
                self.push(Value::Str(target));

                Ok(true)
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

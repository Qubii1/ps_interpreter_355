use super::value::Value;
use super::exec::Interpreter;

impl Interpreter
{
    pub fn try_builtin(&mut self, name: &str) -> Result<bool, String>
    {
        match name
        {
            "add" =>
            {
                let b = self.pop()?;

                let a = self.pop()?;

                self.push(match (a, b)
                {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                    _ => return Err("Type error in add".to_string()),
                });

                Ok(true)
            }

            "sub" =>
            {
                let b = self.pop()?;

                let a = self.pop()?;

                self.push(match (a, b)
                {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                    _ => return Err("Type error in sub".to_string()),
                });

                Ok(true)
            }

            "dup" =>
            {
                let top = self.peek().ok_or("Operand stack underflow")?.clone();

                self.push(top);

                Ok(true)
            }

            "exch" =>
            {
                let b = self.pop()?;

                let a = self.pop()?;

                self.push(b);

                self.push(a);

                Ok(true)
            }

            "pop" =>
            {
                self.pop()?;

                Ok(true)
            }

            "def" =>
            {
                let value = self.pop()?;

                let name = self.pop()?;

                if let Value::Name(n) = name
                {
                    self.dict.define(&n, value);

                    Ok(true)
                }
                else
                {
                    Err("def expects a literal name".to_string())
                }
            }
            
            _ => Ok(false),
        }
    }
}

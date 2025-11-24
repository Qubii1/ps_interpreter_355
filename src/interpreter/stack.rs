use super::value::Value;

#[derive(Debug)]
pub struct OperandStack
{
    items: Vec<Value>,
}

impl OperandStack
{
    pub fn new() -> Self
    {
        Self
        { 
            items: Vec::new()
        }
    }

    pub fn push(&mut self, v: Value)
    {
        self.items.push(v);
    }

    pub fn pop(&mut self) -> Result<Value, String>
    {
        self.items.pop().ok_or_else(|| "Operand stack underflow".to_string())
    }

    pub fn peek(&self) -> Option<&Value>
    {
        self.items.last()
    }

    pub fn len(&self) -> usize
    {
        self.items.len()
    }

    pub fn clear(&mut self)
    {
        self.items.clear();
    }

    pub fn snapshot(&self) -> Vec<Value>
    {
        self.items.clone()
    }
}
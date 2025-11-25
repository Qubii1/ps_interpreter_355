// -----------------------------------------------------------------------------
// File: stack.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Defines the OperandStack type and implements different operations it can handle.
// -----------------------------------------------------------------------------

use super::value::Value;

#[derive(Debug)]
// OperandStack type that contains an items attribute that 
// holds a vector of Values.
pub struct OperandStack
{
    items: Vec<Value>,
}


impl OperandStack
{

    // Constructor for the OperandStack
    pub fn new() -> Self
    {
        Self
        { 
            items: Vec::new()
        }
    }

    // Push method for pushing something to the OperandStack
    pub fn push(&mut self, v: Value)
    {
        self.items.push(v);
    }

    // Pop method for popping something off the OperandStack
    // Will return an error if the stack is empty and this is invoked.
    pub fn pop(&mut self) -> Result<Value, String>
    {
        self.items.pop().ok_or_else(|| "Operand stack underflow".to_string())
    }

    // Peek method for getting the value at the top of the OperandStack
    pub fn peek(&self) -> Option<&Value>
    {
        self.items.last()
    }

    // Len method for getting the size of the OperandStack.
    pub fn len(&self) -> usize
    {
        self.items.len()
    }

    // Clear method for clearing all the items in the OperandStack.
    pub fn clear(&mut self)
    {
        self.items.clear();
    }

    // Snapshot method that gets the items in the OperandStack
    pub fn snapshot(&self) -> Vec<Value>
    {
        self.items.clone()
    }
}
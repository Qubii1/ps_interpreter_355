// -----------------------------------------------------------------------------
// File: dict.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Defines the dictionary stack with different scope mode lookup 
// methods.
// -----------------------------------------------------------------------------

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use super::value::Value;

pub type Dict = Rc<RefCell<HashMap<String, Value>>>;
pub type EnvRef = Rc<RefCell<Vec<Dict>>>;

#[derive(Debug)]
pub struct DictStack
{
    stack: EnvRef,
}

impl DictStack
{
    pub fn new() -> Self
    {

        // Create an empty global dictionary and wrap it in a Rc<RefCell>>
        let global_dict: Dict = Rc::new(RefCell::new(HashMap::new()));

        // Probably the most complicated piece of code.
        // Breaking it down in pieces here is what it does:
        // 1) Vec<HashMap<String, Value>>: This piece will create a vector of dictionaries
        // and it ensures that when created it will always have at least one
        // empty dictionary, for example the global scope. This mimics how postscript
        // works, where each dictionary is one scope frame, and the whole vector is the dictionary stack
        // 2) RefCell<Vec<HashMap<String, Value>>>: Wraps that previous vector of dictionaries
        // in a RefCell so it can get mutable access and immutable access.
        // 3) Rc<RefCell<Vec<HashMap<String, Value>>>>: Finally wrap that RefCell in a Rc, so
        // multiple parts of the interpreter can share the environment, for example
        // the main interpreter, or the procedures that capture this environment
        // for lexical scoping.
        let env: EnvRef = Rc::new(RefCell::new(vec![global_dict]));
        Self { stack: env }
    }

    // Exposes the environment and returns a clone of the Rc; doesnt
    // deep copy the whole vector of dictionaries. All EnvRef values 
    // will point to the same underlying Vec<Dict>. This is important because
    // When you capture an EnvRef inside a Procedure, that procedure can later 
    // be executed using the same dictionary stack that existed 
    // when it was defined (lexical scoping behavior).
    pub fn env(&self) -> EnvRef
    {
        Rc::clone(&self.stack)
    }

    // Inserts a variable definition into the top dictionary on the stack.
    // Equivalent to doing something like /x 10 def in postscript.
    // DictStack itself is immutably borrowed, but we can 
    // still mutate the inner vec because stack is a RefCell. 
    // This is Rust’s interior mutability pattern.
    pub fn define(&self, name: &str, value: Value)
    {
        // This locks the RefCell for mutable access at runtime.
        // env is now like &mut Vec<Dict>
        let mut env = self.stack.borrow_mut();

        // env.last_mut() will get a mutable reference to the top
        // dictionary on the stack, and inserts the name and value to the stack.
        env.last_mut().unwrap().borrow_mut().insert(name.to_string(), value);
    }

    // Dynamic scoping
    pub fn lookup_dynamic(&self, name: &str) -> Option<Value>
    {
        // .borrow() will actually get an immutable reference to the Dictionary stack.
        let env = self.stack.borrow();

        // This implements the dynamic scoping rule:
        // Search all dictionary frames from top to bottom, returning the first match.
        // Will return Some(&Value) if found, None if not found.
        // env.iter().rev():
        // Iterate over the vec from top to bottom (last to first), so more recent scopes shadow older ones.
        for dict_ref in env.iter().rev()
        {
            let dict = dict_ref.borrow();

            if let Some(v) = dict.get(name)
            {
                // Clone the value so we return an owned copy.
                return Some(v.clone());
            }
        }
        None
    }

    // Lexical scoping
    pub fn lookup_lexical(&self, name: &str, ctx: &EnvRef) -> Option<Value>
    {
        // Lexical scoping will use the captured environment (ctx) not self.stack
        // where the procedure was defined.
        let env = ctx.borrow();
        for dict_ref in env.iter().rev()
        {
            let dict = dict_ref.borrow();

            if let Some(v) = dict.get(name)
            {
                return Some(v.clone());
            }
        }
        None
    }
}

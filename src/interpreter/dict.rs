use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use super::value::Value;

pub type Dict = HashMap<String, Value>;
pub type EnvRef = Rc<RefCell<Vec<Dict>>>;

#[derive(Debug)]
pub struct DictStack {
    stack: EnvRef,
}

impl DictStack {
    pub fn new() -> Self {
        let env = Rc::new(RefCell::new(vec![HashMap::new()]));
        Self { stack: env }
    }

    pub fn env(&self) -> EnvRef {
        Rc::clone(&self.stack)
    }

    pub fn define(&self, name: &str, value: Value) {
        let mut env = self.stack.borrow_mut();
        env.last_mut().unwrap().insert(name.to_string(), value);
    }

    pub fn lookup_dynamic(&self, name: &str) -> Option<Value> {
        let env = self.stack.borrow();
        for dict in env.iter().rev() {
            if let Some(v) = dict.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn lookup_lexical(&self, name: &str, ctx: &EnvRef) -> Option<Value> {
        let env = ctx.borrow();
        for dict in env.iter().rev() {
            if let Some(v) = dict.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}

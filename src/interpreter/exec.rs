use super::stack::OperandStack;
use super::dict::{DictStack, EnvRef};
use super::tokenizer::{Token, tokenize};
use super::value::Value;
use super::scope::ScopeMode;

pub type InterpreterResult = Result<(), String>;

pub struct Interpreter
{
    pub opstack: OperandStack,
    pub dict: DictStack,
    pub scope_mode: ScopeMode,
}

impl Interpreter
{
    pub fn new(scope: ScopeMode) -> Self 
    {
        Self
        {
            opstack: OperandStack::new(),
            dict: DictStack::new(),
            scope_mode: scope,
        }
    }

    pub fn interpret(&mut self, src: &str) -> InterpreterResult 
    {
        let tokens = tokenize(src)?;
        self.exec_tokens(&tokens, None)
    }

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
                        continue;
                    }

                    let resolved = match self.scope_mode
                    {
                        ScopeMode::Dynamic => self.dict.lookup_dynamic(name),
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
                            self.exec_tokens(&body, captured_env)?;
                        }
                        _ => self.opstack.push(resolved),
                    }
                }
            }
        }
        Ok(())
    }

    // Convenience:
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
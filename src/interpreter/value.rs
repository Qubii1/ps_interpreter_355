use super::dict::EnvRef;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Real(f64),
    Bool(bool),
    Str(String),
    Name(String),
    Procedure(Vec<super::tokenizer::Token>, Option<EnvRef>),
}

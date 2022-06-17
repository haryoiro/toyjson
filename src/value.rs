use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Value {
    String(String),
    Number(usize),
    Boolean(bool),
    Object(Vec<(String, Box<Value>)>),
    Array(Vec<Value>),
    Null,
    Error(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Object(o) => write!(f, "{:?}", o),
            Value::Array(v) => {
                write!(f, "[")?;
                for i in v {
                    write!(f, "{}", i)?;
                }
                write!(f, "]")
            }
            Value::Null => write!(f, "null"),
            Value::Error(e) => write!(f, "{}", e),
        }
    }
}

use std::fmt::{Display, Error, Formatter};

enum Value {
    String(String),
    Number(usize),
    Boolean(Boolean),
    Object(Object),
    Array(Array),
}

enum Boolean {
    True,
    False,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Boolean::True => write!(f, "true"),
            Boolean::False => write!(f, "false"),
        }
    }
}

struct Object((String, Box<Value>));
struct Array(Vec<Value>);

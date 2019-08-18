use std::fmt;
use std::collections::HashMap;

pub struct Object {
    pub map: HashMap<String, Value>,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n").expect("failed to write object");
        for (key, value) in &self.map {
            write!(f, "  {}: {}\n", key, value).expect("failed to write object");
        };
        write!(f, "}}\n")
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub enum Value {
    Object(Object),
    Number(f64),
    String(String),
    Array(Vec<Value>)
}

impl fmt::Display for Value {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Value::Object(object) => object.to_string(),
            Value::Array(array) => array.iter()
                                     .fold(String::new(), |res, value| format!("{}, {}", res, value.to_string())),
            Value::String(string) => string.to_string(),
            Value::Number(number) => number.to_string()
        };
        write!(f, "{}", value)
    }
}




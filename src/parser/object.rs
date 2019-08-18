use std::fmt;
use std::collections::HashMap;

pub struct Object {
    pub map: HashMap<String, Value>,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object: {{\n").expect("cant write debug string");
        for (key, value) in &self.map {
            write!(f, "  {}: {:?}\n", key, value).expect("cant write debug string");
        };
        write!(f, "}}\n")
    }
}

pub enum Value {
    Object(Object),
    Number(f64),
    String(String),
    Array(Vec<Value>)
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Value::Object(_) => "Object".to_string(),
            Value::Array(_) => "Array".to_string(),
            Value::String(string) => string.to_string(),
            Value::Number(number) => number.to_string()
        };
        write!(f, "{}", value)
    }
}




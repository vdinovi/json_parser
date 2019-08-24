extern crate itertools;
use std::fmt;
use std::collections::HashMap;
use itertools::Itertools;

pub struct Object {
    pub map: HashMap<String, Value>,
}


impl Object {
    pub fn to_basic_string(&self) -> String {
        [
            "{",
            self.map.iter().map(|(key, value)| format!("\"{}\":{}", key, value)).join(",").as_str(),
            "}"
        ].join("")
    }

    pub fn to_pretty_string(&self, depth: usize) -> String {
        let indent: String = std::iter::repeat(" ").take(depth * 2).join("");
        [
            format!("{{\n"),
            self.map.iter().map(|(key, value)| format!("{}  \"{}\": {}", indent, key, value.to_pretty_string(depth + 1))).join(",\n"),
            format!("\n{}}}", indent),
        ].join("")
    }
}


impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_pretty_string(0))
    }
}


impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_basic_string())
    }
}

pub struct Array {
    pub values: Vec<Value>
}

impl Array {
    pub fn to_basic_string(&self) -> String {
        [
            "[",
            self.values.iter().map(|value| value.to_string()).join(",").as_str(),
            "]"
        ].join("")
    }

    pub fn to_pretty_string(&self, depth: usize) -> String {
        let indent: String = std::iter::repeat(" ").take(depth * 2).join("");
        [
            format!("[\n"),
            self.values.iter().map(|value| format!("{}  {}", indent, value.to_pretty_string(depth + 1))).join(",\n"),
            format!("\n{}]", indent)
        ].join("")
    }


}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_pretty_string(0))
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_basic_string())
    }
}

pub enum Value {
    Object(Object),
    Number(f64),
    String(String),
    Array(Array),
    Keyword(String)
}

impl fmt::Display for Value {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Value::Object(object) => object.to_string(),
            Value::Array(array) => array.to_string(),
            Value::String(string) => format!("\"{}\"", string),
            Value::Number(number) => number.to_string(),
            Value::Keyword(string) => string.to_string()
        };
        write!(f, "{}", value)
    }
}

impl Value {
    pub fn to_pretty_string(&self, depth: usize) -> String {
        match self {
            Value::Object(object) => object.to_pretty_string(depth),
            Value::Array(array) => array.to_pretty_string(depth),
            Value::String(string) => format!("\"{}\"", string),
            Value::Number(number) => number.to_string(),
            Value::Keyword(string) => string.to_string()
        }
    }
}





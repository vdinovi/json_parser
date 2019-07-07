use std::collections::HashMap;
use crate::tokenizer::{Token, TokenType};
use std::fmt;
use std::error;

enum Value {
    //Object(Object),
    Number(f64),
    //String(String)
}

pub struct Object {
    map: HashMap<String, Value>,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object: {{\n").expect("cant write debug string");
        for (key, value) in &self.map {
            let val: String = match value {
                //Value::Object(_) => "Object".to_string(),
                //Value::String(string) => string.to_string(),
                Value::Number(number) => number.to_string()
            };
            write!(f, "  {}: {}\n", key, val).expect("cant write debug string");
        };
        write!(f, "}}\n")
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    line_num: u32,
    msg: String
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError (line {}): {}", self.line_num, self.msg)
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Object, ParseError> {
    let mut tok_iter: std::vec::IntoIter<Token> = tokens.into_iter();
    parse_object(&mut tok_iter)
}

fn parse_object(tok_iter: &mut std::vec::IntoIter<Token>) -> Result<Object, ParseError> {
    match tok_iter.next() {
        Some(token) => match token.tok_type {
            TokenType::LBrace => match parse_key_values(tok_iter) {
                Ok(map) => Ok(Object { map }),
                Err(e) => Err(e)
            }
            _ => return Err(
                ParseError { line_num: token.line_num, msg: format!("unexpected token {:?}", token).to_string() }
            )
        },
        None => return Err(
            ParseError { line_num: 0, msg: "unexpected end of token stream".to_string() }
        )
    }
}

fn parse_key_values(_tok_iter: &mut std::vec::IntoIter<Token>) -> Result<HashMap<String, Value>, ParseError> {
    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert("stub".to_string(), Value::Number(1.0));
    Ok(map)
}

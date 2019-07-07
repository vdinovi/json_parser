use std::collections::HashMap;
use crate::tokenizer::{Token, TokenType, TokenData};
use std::fmt;
use std::error;

enum Value {
    Object(Object),
    Number(f64),
    String(String),
    Array(Vec<Value>)
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
                Value::String(string) => string.to_string(),
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

fn parse_key_values(tok_iter: &mut std::vec::IntoIter<Token>) -> Result<HashMap<String, Value>, ParseError> {
    let mut map: HashMap<String, Value> = HashMap::new();
    loop {
        // Match string
        let key = match tok_iter.next() {
            Some(token) => match &token.tok_type {
                TokenType::String => match &token.data {
                    TokenData::String(string) => string.to_string(),
                    other => {
                        let error_str = format!("unexpected token {:?}", token).to_string();
                        return Err(ParseError { line_num: token.line_num, msg: error_str })
                    }
                }
                TokenType::RBrace => return Ok(map),
                other => {
                    let error_str = format!("unexpected token {:?}", token).to_string();
                    return Err(ParseError { line_num: token.line_num, msg: error_str })
                }
            },
            None => {
                let error_str = format!("unexpected end of token stream").to_string();
                return Err(ParseError { line_num: 0, msg: error_str } )
            }
        };
        // Match colon
        match tok_iter.next() {
            Some(token) => match &token.tok_type {
                TokenType::Colon => (),
                other => {
                    let error_str = format!("expected colon, found {:?}", token).to_string();
                    return Err(ParseError { line_num: token.line_num, msg: error_str })
                }
            },
            None => {
                let error_str = "unexpected end of token stream".to_string();
                return Err(ParseError { line_num: 0, msg: error_str })
            }
        }
        // Match RHS
        let value = match tok_iter.next() {
            Some(token) => match &token.tok_type {
                TokenType::LBrace => match parse_object(tok_iter) {
                    Ok(object) => Value::Object(object),
                    Err(e) => return Err(e)
                },
                TokenType::LBracket => match parse_array(tok_iter) {
                    Ok(array) => Value::Array(array),
                    Err(e) => return Err(e)
                },
                TokenType::Number | TokenType::String => match token.data {
                    TokenData::String(string) => Value::String(string),
                    TokenData::Number(number) => Value::Number(number)
                },
                other => {
                    let error_str = format!("unexpected token {:?}", token.tok_type).to_string();
                    return Err(ParseError { line_num: 0, msg: error_str })
                }

            },
            None => {
                let error_str = "unexpected end of token stream".to_string();
                return Err(ParseError { line_num: 0, msg: error_str })
            }
        };
        match map.insert(key, value) {
            Ok(v) => Ok(map),
            Err(e) => Err(e)
        }
    }
}

fn parse_array(tok_iter: &mut std::vec::IntoIter<Token>) -> Result<Vec<Value>, ParseError> {
    let values: Vec<Value> = Vec::new();
    loop {
        match tok_iter.next() {
            Some(token) => match token.tok_type {
                TokenType::Number | TokenType::String => match token.data {
                    TokenData::String(string) => values.push(Value::String(string)),
                    TokenData::Number(number) => values.push(Value::Number(number))
                },
                TokenType::RBracket => break,
                other => {
                    let error_str = format!("unexpected token {:?}", token).to_string();
                    return Err(ParseError { line_num: 0, msg: error_str })

                }
            },
            None => {
                let error_str = "unexpected end of token stream".to_string();
                return Err(ParseError { line_num: 0, msg: error_str })
            }
        }
    };
    Ok(values)
}

/*use std::collections::HashMap;
use crate::tokenizer::{Token, TokenType};
use fmt;

enum Terminal {
    String,
    Number
}

enum Value {
    Object,
    Terminal
}

struct Object {
    map: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
struct ParseError {
    line: i32,
    msg: String
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError (line {}): {}", self.line, self.msg)
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Object, ParseError> {
    let mut tok_iter: std::vec::IntoIter<Token> = tokens.into_iter();
    let mut objects: Vec<Object> = Vec::new();
    let mut rootObject: Object  = Object {};

    match tok_iter.next() {
        Some(token) => match token {
            LBrace =>  parse_object(token, &mut tok_iter),
            _ => ParseError { 
        },
        None        => break
    }
}

fn parse_object(token: Token, tok_iter: &mut std::vec::IntoIter<Token>) -> Object {
    let map: HashMap<String, Object> = match token {
        TokenType::LBrace((_, data)) => parse_key_values(tok_iter),
        other => {
            println!("Parse Error: unexpected token ({}) expecting LBrace", other);
            std::process::exit(1);
        }
    };
    Object{ map: map }
}

fn parse_key_values(tok_iter: &mut std::vec::IntoIter<Token>) -> HashMap<String, ObjectValue> {
    loop {
        let key: String = match tok_iter.next() {
            Some(token) => match token {
                Token(TokenType::String, s) => s,
                other => {
                    println!("Parse Error: unexpected token ({}) expecting LBrace", other);
                    std::process::exit(1);
                }
            },
            None => {
                println!("Parse Error: unexpected end of tokens");
                std::process::exit(1);
            }
        };
        // String, Colon, Object, repeat
    }
}

*/

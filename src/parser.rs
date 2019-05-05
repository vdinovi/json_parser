use std::collections::HashMap;
use crate::tokenizer::{Token, TokenType};

enum ObjectValue {
    String(Some(String)),
    Number(Some(f64)),
    Array(Some(Vec<ObjectTypes>)),
    Object(Some(Object)),
    Null(None)
}

// I don't think this is right, it needs to be a variant: 
pub struct Object {
    map: HashMap<String, ObjectValue>
}

pub fn parse(tokens: &Vec<Token>) -> Vec<Object> {
    let mut tok_iter: std::vec::IntoIter<Token> = tokens.into_iter();
    let mut objects: Vec<Object> = Vec::new();

    loop {
        let object: Object = match tok_iter.next() {
            Some(token) => parse_object(token, &mut tok_iter),
            None        => break
        };
        objects.push(object);
    };
    objects
}

fn parse_object(token: Token, tok_iter: &mut std::vec::IntoIter<Token>) -> Object {
    let map: HashMap<String, Object> = match token {
        LBrace(_, data) => parse_key_values(tok_iter),
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



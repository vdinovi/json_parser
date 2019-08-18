extern crate backtrace;
pub mod error;
pub mod object;

use std::collections::HashMap;
use crate::token::token::{Token, TokenType, TokenData};
use crate::token::stream::TokenStream;
use error::ParseError;
use object::{Object, Array, Value};

pub fn parse(tokens: Vec<Token>) -> Result<Object, ParseError> {
    parse_object(&mut TokenStream::new(&tokens))
}

fn parse_object(stream: &mut TokenStream) -> Result<Object, ParseError> {
    stream.consume(TokenType::LBrace)?;
    let map = parse_key_values(stream)?;
    stream.consume(TokenType::RBrace)?;
    Ok(Object { map })
}

fn parse_key(stream: &mut TokenStream) -> Result<String, ParseError> {
    let token: &Token = stream.consume(TokenType::String)?;
    match &token.data {
        TokenData::String(string) => Ok(string.clone()),
        TokenData::Number(float) => Ok(float.to_string()),
        TokenData::None => Err(ParseError::new(&format!("expected identifier, found {:?}", token), token.line_num))
    }
}

fn parse_value(stream: &mut TokenStream) -> Result<Value, ParseError> {
    match stream.peek()?.tok_type {
        TokenType::LBrace => {
            let object: Object = parse_object(stream)?;
            return Ok(Value::Object(object))
        },
        TokenType::LBracket => {
            let array: Vec<Value> = parse_array(stream)?;
            return Ok(Value::Array(Array { values: array }))
        },
        _ => {
            let token: &Token = stream.next()?;
            match token.tok_type {
                TokenType::Number | TokenType::String => match &token.data {
                    TokenData::String(string) => Ok(Value::String(string.clone())),
                    TokenData::Number(number) => Ok(Value::Number(number.clone())),
                    TokenData::None => Err(ParseError::new("token is missing token data", token.line_num))
                },
                _ => Err(ParseError::new(&format!("unexpected token {:?}", token.tok_type), token.line_num))
            }

        }
    }
}

fn parse_array(stream: &mut TokenStream) -> Result<Vec<Value>, ParseError> {
    let mut values: Vec<Value> = Vec::new();
    stream.consume(TokenType::LBracket)?;

    loop {
        match stream.peek()?.tok_type {
            TokenType::RBracket => break,
            _ => ()
        };
        values.push(parse_value(stream)?);
        match stream.peek()?.tok_type {
            TokenType::Comma => { stream.next()?; },
            _ => ()
        };
    };
    stream.consume(TokenType::RBracket)?;
    Ok(values)
}

fn parse_key_values(stream: &mut TokenStream) -> Result<HashMap<String, Value>, ParseError> {
    let mut map: HashMap<String, Value> = HashMap::new();
    loop {
        match stream.peek()?.tok_type {
            TokenType::RBrace => break,
            _ => ()
        };
        let key: String = parse_key(stream)?;
        stream.consume(TokenType::Colon)?;
        let value: Value = parse_value(stream)?;
        match stream.peek()?.tok_type {
            TokenType::Comma => { stream.next()?; },
            _ => ()
        };
        map.insert(key, value);
    };
    Ok(map)
}

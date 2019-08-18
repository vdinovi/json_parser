extern crate backtrace;

use backtrace::Backtrace;
use std::collections::HashMap;
use crate::tokenizer::{Token, TokenType, TokenData};
use std::fmt;
use std::error;

struct TokenStream<'a> {
    tokens: &'a [Token],
    size: usize,
    position: usize
}

impl<'a> TokenStream<'a> {
    fn new(vec: &'a Vec<Token>) -> TokenStream<'a> {
        let tok_slice: &[Token] = vec.as_slice();
        TokenStream {
            tokens: tok_slice,
            size: tok_slice.len(),
            position: 0
        }
    }

    fn next(&mut self) -> Result<&Token, ParseError> {
        if self.position + 1 < self.size {
            let token: &Token = &self.tokens[self.position];
            self.position += 1;
            Ok(token)
        } else {
            Err(ParseError::new("Cannot advance beyond end of token stream", 0))
        }
    }

    fn consume(&mut self, tok_type: TokenType) -> Result<&Token, ParseError> {
        let token: &Token = self.next()?;
        if token.tok_type.ordinal() == tok_type.ordinal() {
            Ok(token)
        } else {
            Err(ParseError::new(&format!("Unexpected Token: expected {:?}, found {:?}", tok_type, token), token.line_num))
        }
    }

    fn peek(&self) -> Result<&Token, ParseError> {
        if self.position > 0 && self.position < self.size {
            Ok(&self.tokens[self.position])
        } else {
            Err(ParseError::new("Cannot peek beyond range of token stream", 0))
        }
    }

    fn backtrack(&self) -> Result<(), ParseError> {
        if self.position > 0 {
            Ok(())
        } else {
            Err(ParseError::new("Cannot backtrack beyond first token", 0))
        }
    }

}

enum Value {
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

pub struct Object {
    map: HashMap<String, Value>,
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

#[derive(Debug, Clone)]
pub struct ParseError {
    line_num: u32,
    msg: String,
    backtrace: Backtrace
}

impl ParseError {
    pub fn new(msg: &str, line_num: u32) -> ParseError {
        ParseError {
            msg: msg.to_string(),
            line_num: line_num,
            backtrace: Backtrace::new()
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError (line {}): {}\n{:?}", self.line_num, self.msg, self.backtrace)
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

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
            return Ok(Value::Array(array))
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
            TokenType::RBrace => { stream.next()?; return Ok(map) },
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
        print!("map: {:?}\n", map);
    }
}

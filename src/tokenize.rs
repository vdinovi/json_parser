use std::fmt;
use std::io::BufRead;

#[derive(Debug)]
enum TokenType {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    String,
    Number,
    Unknown
}

enum TokenData {
    Number(f64),
    String(String)
}

enum TokenizedResult {
    None,
    One(Token),
    Many(Vec<Token>)
}

pub struct Token {
    r#type: TokenType,
    data:   Option<TokenData>
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       let data = match &self.data {
            Some(d) => match d {
                TokenData::Number(n) => Some(n.to_string()),
                TokenData::String(s) => Some(s.clone()),
            }
            None    => None
        };
        write!(f, "Token{{type: {:?}, data: {:?}}}", self.r#type, data)
    }
}

pub fn tokenize<R: BufRead>(r: &mut R) -> Vec<Token> {
    let mut byte_buf: Vec<u8> = Vec::new();
    let mut tok_buf: Vec<Token> = Vec::new();
    match r.read_to_end(&mut byte_buf) {
        Ok(v) => v,
        Err(e) => {
            println!("Read Error: {}", e);
            std::process::exit(1);
        }
    };
    let mut byte_iter: std::vec::IntoIter<u8> = byte_buf.into_iter();

    loop {
        match byte_iter.next() {
            Some(byte) => {
                match tokenize_main(byte, &mut byte_iter) {
                    TokenizedResult::None       => (),
                    TokenizedResult::One(tok)   => tok_buf.push(tok),
                    TokenizedResult::Many(mut toks) => tok_buf.append(&mut toks)
                }
            },
            None => break
        };
    };
    tok_buf
}



fn tokenize_main(byte: u8, byte_iter: &mut std::vec::IntoIter<u8>) -> TokenizedResult {
    match byte {
        b' ' | b'\n' | b'\t'  => TokenizedResult::None,
        b'{'  => TokenizedResult::One(Token{ r#type: TokenType::LBrace,    data: None  }),
        b'}'  => TokenizedResult::One(Token{ r#type: TokenType::RBrace,    data: None  }),
        b'['  => TokenizedResult::One(Token{ r#type: TokenType::LBracket,  data: None  }),
        b']'  => TokenizedResult::One(Token{ r#type: TokenType::RBracket,  data: None  }),
        b':'  => TokenizedResult::One(Token{ r#type: TokenType::Colon,     data: None  }),
        b','  => TokenizedResult::One(Token{ r#type: TokenType::Comma,     data: None  }),
        b'"'  => TokenizedResult::One(tokenize_string(byte_iter)),
        b'1' ... b'9' | b'.'  => TokenizedResult::Many(tokenize_number(byte, byte_iter)),
        _     => TokenizedResult::One(Token{ r#type: TokenType::Unknown, data: None  })
    }
}

fn tokenize_string(byte_iter: &mut std::vec::IntoIter<u8>) -> Token {
    let mut data_buf: Vec<u8> = Vec::new();
    loop {
        match byte_iter.next() {
            Some(byte) => match byte {
                b'"' => break,
                _    => data_buf.push(byte)
            }
            None => ()
        };
    };
    let data_str = match String::from_utf8(data_buf) {
        Ok(s) => s,
        Err(e) => {
            println!("Tokenize Error: {}", e);
            std::process::exit(1);
        }
    };
    Token{ r#type: TokenType::String, data: Some(TokenData::String(data_str)) }
}

fn tokenize_number(byte: u8, byte_iter: &mut std::vec::IntoIter<u8>) -> Vec<Token> {
    let mut data_buf: Vec<u8> = vec![byte];
    loop {
        match byte_iter.next() {
            Some(byte) => match byte {
                b'1' ... b'9' | b'.'  => data_buf.push(byte),
                b' ' | b'\n' | b'\t'  => (),
                b','                  => break,
                other                 => {
                    println!("Tokenize Error: unexpected token ({}) while parsing number (expecting comma)", other);
                    std::process::exit(1);
                }
            }
            None => ()
        };
    };
    let data_str = match String::from_utf8(data_buf) {
        Ok(s) => s,
        Err(e) => {
            println!("Tokenize Error: {}", e);
            std::process::exit(1);
        }
    };
    let data_num: f64 = match data_str.parse::<f64>() {
        Ok(f) => f,
        Err(e) => {
            println!("Tokenize Error: {}", e);
            std::process::exit(1);
        }
    };
    vec![
        Token{ r#type: TokenType::Number, data: Some(TokenData::Number(data_num)) },
        Token{ r#type: TokenType::Comma,  data: None },
    ]
}

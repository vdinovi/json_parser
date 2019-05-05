use std::env;
use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fmt;


#[derive(Debug)]
enum TokenType {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    String,
    //Number,
    Unknown
}

struct Token {
    r#type: TokenType,
    data:   Option<Vec<u8>>
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token{{type: {:?}, data: {:?}}}", self.r#type, self.data)
    }
}

fn tokenize<R: BufRead>(r: &mut R) -> Vec<Token> {
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
                    Some(token) => tok_buf.push(token),
                    None        => ()
                }
            },
            None => break
        };
    };
    tok_buf
}

fn tokenize_main(byte: u8, byte_iter: &mut std::vec::IntoIter<u8>) -> Option<Token> {
    match byte {
        b' ' | b'\n' | b'\t'  => None,
        b'{'  => Some(Token{ r#type: TokenType::LBrace,    data: None  }),
        b'}'  => Some(Token{ r#type: TokenType::RBrace,    data: None  }),
        b'['  => Some(Token{ r#type: TokenType::LBracket,  data: None  }),
        b']'  => Some(Token{ r#type: TokenType::RBracket,  data: None  }),
        b':'  => Some(Token{ r#type: TokenType::Colon,     data: None  }),
        b'"'  => Some(tokenize_string(byte_iter)),
        _     => Some(Token{ r#type: TokenType::Unknown, data: None  })
    }
}

fn tokenize_string(byte_iter: &mut std::vec::IntoIter<u8>) -> Token {
    byte_iter.next(); // skip opening quote
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
    Token{ r#type: TokenType::String, data: Some(data_buf) }
}

fn main() {
    const usage: &str = "usage: json_parser path/to/json/file";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", usage);
        std::process::exit(1);
    }

    let rel_path = PathBuf::from(args[1].to_string());
    let full_path = match rel_path.canonicalize() {
        Ok(v) => v,
        Err(e) => {
            println!("File Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("Parsing file: {:?}", full_path);
    let file = match File::open(full_path) {
        Ok(v) => v,
        Err(e) => {
            println!("File Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut reader = BufReader::new(file);
    let tokens: Vec<Token> = tokenize(&mut reader);
    for token in tokens {
        println!("{:?}", token);
    }
}

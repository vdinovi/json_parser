use std::env;
use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fmt;


#[derive(Debug)]
enum TokenType {
    LBrace,
    RBrace,
    Colon,
    String,
    Number,
    Unknown
}

struct Token {
    r#type: TokenType,
    data:   Option<Vec<u8>>
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token{{type: {:?}}}", self.r#type)
    }
}

fn tokenize<R: BufRead>(r: &mut R) -> Vec<Token> {
    let mut byte_buf: Vec<u8> = vec![1, 2, 3];
    let mut tok_buf: Vec<Token> = vec![];
    r.read_to_end(&mut byte_buf);
    let mut byte_iter: std::vec::IntoIter<u8> = byte_buf.into_iter();

    loop {
        match byte_iter.next() {
            Some(byte) => tokenize_main(byte, &mut byte_iter, &mut tok_buf),
            None => break
        };
    };
    tok_buf
}

fn tokenize_main<'a>(byte: u8, byte_iter: &'a std::vec::IntoIter<u8>, tok_buf: &mut Vec<Token>) -> &'a std::vec::IntoIter<u8> {
    match byte {
        b' ' | b'\n' | b'\t'  => (),
        b'{'  => tok_buf.push(Token{ r#type: TokenType::LBrace, data: None  }),
        b'}'  => tok_buf.push(Token{ r#type: TokenType::RBrace, data: None  }),
        b'"'  => { tokenize_string(&mut byte_iter, &mut tok_buf); () },
        _     => tok_buf.push(Token{ r#type: TokenType::Unknown, data: None  })
    };
    byte_iter
}

fn tokenize_string<'a>(byte_iter: &'a std::vec::IntoIter<u8>, tok_buf: &mut Vec<Token>) -> &'a std::vec::IntoIter<u8> {
    let mut data_buf: Vec<u8> = vec![];
    loop {
        match byte_iter.next() {
            Some(byte) => match byte {
                b'"' => break,
                _    => data_buf.push(byte)
            }
            None => ()
        };
    };
    tok_buf.push(Token{ r#type: TokenType::String, data: Some(data_buf) });
    byte_iter
}

fn main() { const usage: &str = "usage: json_parser path/to/json/file";
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
    println!("{:?}", tokens);
}

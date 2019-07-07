use std::fmt;
use std::io::BufRead;
use std::error;

#[derive(Debug)]
pub enum TokenType {
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

pub enum TokenData {
    None,
    Number(f64),
    String(String)
}

enum TokenizedResult {
    None,
    One(Token),
    Many(Vec<Token>)
}

pub struct Token {
    pub tok_type: TokenType,
    pub data:     TokenData,
    pub line_num: u32
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       let data = match &self.data {
           TokenData::Number(n) => Some(n.to_string()),
           TokenData::String(s) => Some(s.clone()),
           TokenData::None      => None
        };
        write!(f, "Token {{type: {:?}, data: {:?}, line_num: {:?}}}", self.tok_type, data, self.line_num)
    }
}

#[derive(Debug, Clone)]
pub struct TokenizerError {
    line_num: u32,
    msg: String
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TokenizerError (line {}): {}", self.line_num, self.msg)
    }
}

impl error::Error for TokenizerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub fn tokenize<R: BufRead>(r: &mut R) -> Result<Vec<Token>, TokenizerError> {
    let mut byte_buf: Vec<u8> = Vec::new();
    let mut tok_buf: Vec<Token> = Vec::new();
    let mut line_num: u32 = 1;
    match r.read_to_end(&mut byte_buf) {
        Ok(v) => v,
        Err(e) => return Err(
            TokenizerError { line_num, msg: format!("Read Error: {}", e).to_string() }
        )
    };
    let mut byte_iter: std::vec::IntoIter<u8> = byte_buf.into_iter();

    loop {
        match byte_iter.next() {
            Some(byte) => {
                match tokenize_main(byte, &mut byte_iter, &mut line_num) {
                    Ok(result) => match result {
                        TokenizedResult::None       => (),
                        TokenizedResult::One(tok)   => tok_buf.push(tok),
                        TokenizedResult::Many(mut toks) => tok_buf.append(&mut toks)
                    },
                    Err(e) => return Err(e)

                }
            },
            None => break
        };
    };
    Ok(tok_buf)
}



fn tokenize_main(byte: u8, byte_iter: &mut std::vec::IntoIter<u8>, line_num: &mut u32) -> Result<TokenizedResult, TokenizerError> {
    match byte {
        b' ' | b'\t'  => Ok(TokenizedResult::None),
        b'\n' => { *line_num += 1; Ok(TokenizedResult::None) },
        b'{'  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::LBrace,    data: TokenData::None, line_num: *line_num })),
        b'}'  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::RBrace,    data: TokenData::None, line_num: *line_num })),
        b'['  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::LBracket,  data: TokenData::None, line_num: *line_num })),
        b']'  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::RBracket,  data: TokenData::None, line_num: *line_num })),
        b':'  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::Colon,     data: TokenData::None, line_num: *line_num })),
        b','  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::Comma,     data: TokenData::None, line_num: *line_num })),
        b'"'  => match tokenize_string(byte_iter, line_num) {
            Ok(token) => Ok(TokenizedResult::One(token)),
            Err(e) => Err(e)
        },
        b'1' ... b'9' | b'.'  => match tokenize_number(byte, byte_iter, line_num) {
            Ok(tokens) => Ok(TokenizedResult::Many(tokens)),
            Err(e) => Err(e)
        },
        // TODO: should return TokenizerError instead of unknown token
        _     => Ok(TokenizedResult::One(Token{ tok_type: TokenType::Unknown, data: TokenData::None, line_num: *line_num }))
    }
}

fn tokenize_string(byte_iter: &mut std::vec::IntoIter<u8>, line_num: &mut u32) -> Result<Token, TokenizerError> {
    let mut data_buf: Vec<u8> = Vec::new();
    loop {
        match byte_iter.next() {
            Some(byte) => match byte {
                b'"'  => break,
                b'\n' => *line_num += 1,
                _     => data_buf.push(byte)
            }
            None => ()
        };
    };
    let data_str = match String::from_utf8(data_buf) {
        Ok(s) => s,
        Err(e) => return Err(
            TokenizerError { line_num: *line_num, msg: e.to_string() }
        )
    };
    Ok(Token{ tok_type: TokenType::String, data: TokenData::String(data_str), line_num: *line_num })
}

fn tokenize_number(byte: u8, byte_iter: &mut std::vec::IntoIter<u8>, line_num: &mut u32) -> Result<Vec<Token>, TokenizerError> {
    let mut data_buf: Vec<u8> = vec![byte];
    loop {
        match byte_iter.next() {
            Some(byte) => match byte {
                b'1' ... b'9' | b'.'  => data_buf.push(byte),
                b' ' | b'\t'          => (),
                b'\n'                 => *line_num += 1,
                b','                  => break,
                unknown               => return Err(
                    TokenizerError { line_num: *line_num, msg: format!("could not parse token '{}'", unknown).to_string() }
                )
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
    Ok(vec![
        Token{ tok_type: TokenType::Number, data: TokenData::Number(data_num), line_num: *line_num },
        Token{ tok_type: TokenType::Comma,  data: TokenData::None , line_num: *line_num },
    ])
}

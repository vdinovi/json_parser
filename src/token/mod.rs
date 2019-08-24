pub mod error;
pub mod stream;
pub mod token;

use std::io::BufRead;
use std::str::Chars;
use error::TokenizerError;
use token::{Token, TokenizedResult, TokenType, TokenData};

pub fn tokenize<R: BufRead>(r: &mut R) -> Result<Vec<Token>, TokenizerError> {
    let mut buffer: String = String::new();
    let mut tok_buf: Vec<Token> = Vec::new();
    let mut line_num: u32 = 1;

    match r.read_to_string(&mut buffer) {
        Ok(v) => v,
        Err(e) => return Err(
            TokenizerError { line_num, msg: format!("Read Error: {}", e).to_string() }
        )
    };
    let mut char_iter: Chars = buffer.chars();
    loop {
        match char_iter.next() {
            Some(char) => {
                match tokenize_main(char, &mut char_iter, &mut line_num) {
                    Ok(result) => match result {
                        TokenizedResult::None           => (),
                        TokenizedResult::One(tok)       => tok_buf.push(tok),
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

fn tokenize_main(ch: char, char_iter: &mut Chars, line_num: &mut u32) -> Result<TokenizedResult, TokenizerError> {
    match ch {
        ' ' | '\t'  => Ok(TokenizedResult::None),
        '\n' => { *line_num += 1; Ok(TokenizedResult::None) },
        '{'  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::LBrace,    data: TokenData::None, line_num: *line_num })),
        '}'  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::RBrace,    data: TokenData::None, line_num: *line_num })),
        '['  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::LBracket,  data: TokenData::None, line_num: *line_num })),
        ']'  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::RBracket,  data: TokenData::None, line_num: *line_num })),
        ':'  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::Colon,     data: TokenData::None, line_num: *line_num })),
        ','  => Ok(TokenizedResult::One(Token{ tok_type: TokenType::Comma,     data: TokenData::None, line_num: *line_num })),
        '"'  => Ok(TokenizedResult::One(tokenize_string(char_iter, line_num)?)),
        'f' | 't' | 'n'  => Ok(TokenizedResult::Many(tokenize_keyword(ch, char_iter, line_num)?)),
        '0' ... '9' | '.' | '+' | '-'  => Ok(TokenizedResult::Many(tokenize_number(ch, char_iter, line_num)?)),
        _ => Ok(TokenizedResult::One(Token{ tok_type: TokenType::Unknown, data: TokenData::None, line_num: *line_num }))
    }
}

fn tokenize_string(char_iter: &mut Chars, line_num: &mut u32) -> Result<Token, TokenizerError> {
    // This could probably be improved
    let mut chars: Vec<char> = Vec::new();
    loop {
        match char_iter.next() {
            Some(ch) => match ch {
                '"'  => break,
                '\n' => *line_num += 1,
                _     => chars.push(ch)
            }
            None => ()
        };
    };
    let data_str: String = chars.into_iter().collect();
    Ok(Token{ tok_type: TokenType::String, data: TokenData::String(data_str), line_num: *line_num })
}

fn tokenize_keyword(ch: char, char_iter: &mut Chars, line_num: &mut u32) -> Result<Vec<Token>, TokenizerError> {
    let mut chars: Vec<char> = vec![ch];
    loop {
        match char_iter.next() {
            Some(ch) => {
                if ch.is_alphabetic() {
                    chars.push(ch);
                } else {
                    break;
                }
            },
            None => ()
        }
    };
    let keyword: String = chars.into_iter().collect();
    let token: Token = match keyword.as_str() {
        "true" => Token{ tok_type: TokenType::True, data: TokenData::None, line_num: *line_num },
        "false" => Token{ tok_type: TokenType::False, data: TokenData::None, line_num: *line_num },
        "null"  => Token{ tok_type: TokenType::Null, data: TokenData::None, line_num: *line_num },
        _ =>  return Err(
            TokenizerError { line_num: *line_num, msg: format!("Unknown keyword word '{}'", keyword).to_string() }
        )
    };
    Ok(vec![
       token,
        Token{ tok_type: TokenType::Comma,  data: TokenData::None , line_num: *line_num },
    ])
}

fn tokenize_number(ch: char, char_iter: &mut Chars, line_num: &mut u32) -> Result<Vec<Token>, TokenizerError> {
    let mut chars: Vec<char> = vec![ch];
    loop {
        match char_iter.next() {
            Some(ch) => match ch {
                '0' ... '9' | '.' | '+' | '-'  => chars.push(ch),
                ' ' | '\t'                     => (),
                '\n'                           => *line_num += 1,
                ','                            => break,
                unknown                        => return Err(
                    TokenizerError { line_num: *line_num, msg: format!("Unexpected non-numeric character '{}'", unknown).to_string() }
                )
            }
            None => ()
        };
    };
    let data_str: String = chars.into_iter().collect();
    let data_num: f64 = match data_str.parse::<f64>() {
        Ok(f) => f,
        Err(e) => {
            return Err(
                TokenizerError { line_num: *line_num, msg: format!("could not parse string '{}' into number: {}", data_str, e).to_string() }
            )
        }
    };
    Ok(vec![
        Token{ tok_type: TokenType::Number, data: TokenData::Number(data_num), line_num: *line_num },
        Token{ tok_type: TokenType::Comma,  data: TokenData::None , line_num: *line_num },
    ])
}

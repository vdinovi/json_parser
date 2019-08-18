use std::fmt;

pub struct Token {
    pub tok_type: TokenType,
    pub data:     TokenData,
    pub line_num: u32
}

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

impl TokenType {
    pub fn ordinal(&self) -> u8 {
        match self {
            TokenType::LBrace   => 0,
            TokenType::RBrace   => 1,
            TokenType::LBracket => 2,
            TokenType::RBracket => 3,
            TokenType::Colon    => 4,
            TokenType::Comma    => 5,
            TokenType::String   => 6,
            TokenType::Number   => 7,
            TokenType::Unknown  => 8
        }
    }
}

pub enum TokenData {
    None,
    Number(f64),
    String(String)
}

pub enum TokenizedResult {
    None,
    One(Token),
    Many(Vec<Token>)
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

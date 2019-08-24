use crate::token::token::{Token, TokenType};
use crate::parser::error::ParseError;

pub struct TokenStream<'a> {
    pub tokens: &'a [Token],
    pub size: usize,
    pub position: usize
}

impl<'a> TokenStream<'a> {
    pub fn new(vec: &'a Vec<Token>) -> TokenStream<'a> {
        let tok_slice: &[Token] = vec.as_slice();
        TokenStream {
            tokens: tok_slice,
            size: tok_slice.len(),
            position: 0
        }
    }

    pub fn next(&mut self) -> Result<&Token, ParseError> {
        if self.position < self.size {
            let token: &Token = &self.tokens[self.position];
            self.position += 1;
            Ok(token)
        } else {
            Err(ParseError::new("Cannot advance beyond end of token stream", 0))
        }
    }

    pub fn consume(&mut self, tok_type: TokenType) -> Result<&Token, ParseError> {
        let token: &Token = self.next()?;
        if token.tok_type.ordinal() == tok_type.ordinal() {
            Ok(token)
        } else {
            Err(ParseError::new(&format!("Unexpected Token: expected {:?}, found {:?}", tok_type, token), token.line_num))
        }
    }

    pub fn peek(&self) -> Result<&Token, ParseError> {
        if self.position < self.size {
            Ok(&self.tokens[self.position])
        } else {
            Err(ParseError::new("Cannot peek beyond range of token stream", 0))
        }
    }
}

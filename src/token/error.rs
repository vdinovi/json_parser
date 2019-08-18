use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct TokenizerError {
    pub line_num: u32,
    pub msg: String
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

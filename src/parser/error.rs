use std::fmt;
use std::error;
use backtrace::Backtrace;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line_num: u32,
    pub msg: String,
    pub backtrace: Backtrace
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

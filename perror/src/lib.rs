use std::fmt;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    title: &'static str,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "title: {}\n
            {}\n",
            self.title, "ParseError"
        )
    }
}

impl ParseError {
    pub fn new(title: &'static str) -> ParseError {
        ParseError { title }
    }
}

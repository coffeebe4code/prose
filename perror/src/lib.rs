use std::fmt;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    title: String,
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
    pub fn new(title: String) -> ParseError {
        ParseError { title }
    }
}

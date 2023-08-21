use std::fmt;

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug, PartialEq, Clone)]
pub struct ParserError {
    title: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "title: {}\n
            {}\n",
            self.title, "ParseError"
        )
    }
}

impl ParserError {
    pub fn new(title: String) -> ParserError {
        ParserError { title }
    }
}

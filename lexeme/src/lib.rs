use token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme<'a> {
    pub token: Token,
    pub contents: &'a str,
}

impl<'a> Lexeme<'a> {
    pub fn new(contents: &'a str, token: Token) -> Self {
        return Lexeme { token, contents };
    }
    pub fn is_num(&self) -> bool {
        match self.token {
            Token::U8
            | Token::U16
            | Token::U32
            | Token::U64
            | Token::I8
            | Token::I16
            | Token::I32
            | Token::I64
            | Token::F32
            | Token::F64
            | Token::D32
            | Token::D64 => true,
            _ => false,
        }
    }
}

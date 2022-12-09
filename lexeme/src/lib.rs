use token::Token;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lexeme<'source> {
    pub token: Token,
    pub contents: &'source str,
}

pub trait Grabable {
    fn maybe_token(&self) -> Option<Token>;
}

impl<'source> Grabable for Option<Lexeme<'source>> {
    fn maybe_token(&self) -> Option<Token> {
        match self {
            Some(lex) => {
                return Some(lex.token);
            }
            None => {
                return None;
            }
        }
    }
}

impl<'source> Lexeme<'source> {
    pub fn new(contents: &'source str, token: Token) -> Self {
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
            | Token::D64
            | Token::Byte
            | Token::Bit => true,
            _ => false,
        }
    }
    pub fn is_val(&self) -> bool {
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
            | Token::D64
            | Token::Byte
            | Token::Bool
            | Token::Char
            | Token::Void
            | Token::Bit => true,
            _ => false,
        }
    }
}

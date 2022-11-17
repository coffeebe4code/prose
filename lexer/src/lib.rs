use logos::{Lexer, Logos, Span};
use std::error::Error;

use tokens::Token;

pub struct ProseLexer<'a> {
    curr_token: Option<Token>,
    lexer: Lexer<'a, Token>,
}
pub struct LexObj {
    token: Token,
    span: Span,
    str_slice: String,
}

trait Lexable {
    fn ret(&mut self) -> Option<LexObj>;
}

impl<'a> Lexable for Lexer<'a, Token> {
    fn ret(&mut self) -> Option<LexObj> {
        let tok = self.next();
        match tok {
            Some(t) => Some(LexObj {
                token: t,
                span: self.span(),
                str_slice: String::from(self.slice()),
            }),
            None => None,
        }
    }
}

impl<'a> ProseLexer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        ProseLexer {
            curr_token: None,
            lexer: Token::lexer(buffer),
        }
    }
    pub fn current_token(&self) -> Option<Token> {
        return self.curr_token;
    }
}

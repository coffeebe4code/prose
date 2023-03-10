use logos::{Lexer, Logos};
use std::ops::Range;
use token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme<'s> {
    pub token: Token,
    pub span: Range<usize>,
    pub slice: &'s str,
}

pub struct ProseLexer<'s> {
    current: Option<Lexeme<'s>>,
    lexer: Lexer<'s, Token>,
}

impl<'s> ProseLexer<'s> {
    pub fn new(buffer: &'s str) -> Self {
        return ProseLexer {
            current: None,
            lexer: Token::lexer(buffer),
        };
    }
    pub fn collect_if(&mut self, token: Token) -> Option<Lexeme<'s>> {
        if self.peek()?.token.is_kind(token) {
            return Some(self.collect());
        }
        return None;
    }
    pub fn collect_of_if(&mut self, token: &[Token]) -> Option<Lexeme<'s>> {
        if self.peek()?.token.is_of_kind(token) {
            return Some(self.collect());
        }
        return None;
    }
    pub fn peek(&mut self) -> Option<Lexeme<'s>> {
        if self.current.is_none() {
            match self.lexer.next() {
                Some(val) => {
                    self.current = Some(Lexeme {
                        token: val,
                        span: self.lexer.span(),
                        slice: self.lexer.slice(),
                    })
                }
                None => self.current = None,
            }
        }
        self.current.clone()
    }
    pub fn has_token_consume(&mut self, token: Token) -> bool {
        match self.peek() {
            Some(lexeme) => {
                if lexeme.token == token {
                    self.collect();
                    return true;
                }
                return false;
            }
            None => false,
        }
    }
    pub fn collect(&mut self) -> Lexeme<'s> {
        let temp = self.current.clone().unwrap();
        self.current = None;
        return temp;
    }
}

use lexeme::Lexeme;
use logos::{Lexer, Logos};
use token::Token;

pub struct ProseLexer<'source> {
    current: Option<Lexeme<'source>>,
    lexer: Lexer<'source, Token>,
}

impl<'source> ProseLexer<'source> {
    pub fn new(buffer: &'source str) -> Self {
        return ProseLexer {
            current: None,
            lexer: Token::lexer(buffer),
        };
    }
    pub fn collect_if(&mut self, token: Token) -> Option<Lexeme<'source>> {
        if self.peek()?.is_kind(token) {
            return Some(self.collect().unwrap());
        }
        return None;
    }
    pub fn collect_of_if(&mut self, token: &[Token]) -> Option<Lexeme<'source>> {
        if self.peek()?.is_of_kind(token) {
            return Some(self.collect().unwrap());
        }
        return None;
    }
    pub fn peek(&mut self) -> Option<Token> {
        if self.current.is_none() {
            self.current = self.lexer.next_lexeme();
        }
        match &self.current {
            Some(val) => {
                return Some(val.token);
            }
            None => None,
        }
    }
    pub fn has_token_consume(&mut self, token: Token) -> bool {
        match self.peek() {
            Some(tok) => {
                if tok == token {
                    self.collect();
                    return true;
                }
                return false;
            }
            None => false,
        }
    }
    pub fn collect(&mut self) -> Option<Lexeme<'source>> {
        let temp = self.current;
        self.current = None;
        return temp;
    }
}

pub trait Lexable<'source> {
    fn next_lexeme(&mut self) -> Option<Lexeme<'source>>;
}

impl<'source> Lexable<'source> for Lexer<'source, Token> {
    fn next_lexeme(&mut self) -> Option<Lexeme<'source>> {
        let tok = self.next();
        match tok {
            Some(t) => Some(Lexeme {
                token: t,
                contents: self.slice(),
            }),
            None => None,
        }
    }
}

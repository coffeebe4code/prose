use lexeme::Lexeme;
use logos::{Lexer, Logos};
use token::Token;

pub struct ProseLexer<'a> {
    current: Option<Lexeme<'a>>,
    lexer: Lexer<'a, Token>,
}

impl<'a> ProseLexer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        return ProseLexer {
            current: None,
            lexer: Token::lexer(buffer),
        };
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
    pub fn collect(&mut self) -> Option<Lexeme<'a>> {
        let temp = self.current.clone();
        self.current = None;
        return temp;
    }
}

pub trait Lexable<'a> {
    fn next_lexeme(&mut self) -> Option<Lexeme<'a>>;
}

impl<'a> Lexable<'a> for Lexer<'a, Token> {
    fn next_lexeme(&mut self) -> Option<Lexeme<'a>> {
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

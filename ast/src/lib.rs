use lexeme::Lexeme;
use tokens::Token;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Body(Vec<Expr<'a>>),
    BinOp(Box<Expr<'a>>, Token, Box<Expr<'a>>),
    UnaryOp(Box<Expr<'a>>, Token),
    Identity(Lexeme<'a>),
    Number(Lexeme<'a>),
    Single(Token),
}

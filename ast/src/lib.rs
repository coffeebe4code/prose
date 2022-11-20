use lexeme::Lexeme;
use tokens::Token;

pub enum Expr<'a> {
    Body(Vec<Expr<'a>>),
    BinOp(Box<Expr<'a>>, Token, Box<Expr<'a>>),
    UnaryOp(Box<Expr<'a>>, Token),
    Identity(Lexeme<'a>),
    Number(Lexeme<'a>),
    Single(Token),
    Symbol(Lexeme<'a>),
}

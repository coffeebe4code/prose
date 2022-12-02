use lexeme::Lexeme;
use token::Token;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Body(Vec<Expr<'a>>),
    BinOp(Box<Expr<'a>>, Token, Box<Expr<'a>>),
    UnaryOp(Box<Expr<'a>>, Token),
    Identity(Lexeme<'a>),
    Number(Lexeme<'a>),
    Single(Token),
}

#[macro_export]
macro_rules! some_expr {
    ($val:ident, $($inner:tt)*) => {
        Some(Box::new(Expr::$val($($inner)*)));
    };
}

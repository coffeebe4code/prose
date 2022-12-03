use lexeme::Lexeme;
use token::Token;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Body(Vec<Expr<'a>>),
    // mutability, identifier, signature, assignment, expr, semicolon
    Assignment(
        bool,
        Box<Expr<'a>>,
        Option<Box<Expr<'a>>>,
        Token,
        Box<Expr<'a>>,
        bool,
    ),
    // identifier, asop, expr, semicolon
    Reassignment(Box<Expr<'a>>, Token, Box<Expr<'a>>, bool),
    // left expr, op, right expr
    BinOp(Box<Expr<'a>>, Token, Box<Expr<'a>>),
    UnaryOp(Box<Expr<'a>>, Token),
    Identity(Lexeme<'a>),
    RetFn(Option<Box<Expr<'a>>>, bool),
    Number(Lexeme<'a>),
    Single(Token),
}

#[macro_export]
macro_rules! some_expr {
    ($val:ident, $($inner:tt)*) => {
        Some(Box::new(Expr::$val($($inner)*)));
    };
}

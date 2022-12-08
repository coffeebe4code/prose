use lexeme::Lexeme;
use token::Token;

#[derive(Debug, PartialEq)]
pub enum Expr<'source> {
    Body(Vec<Expr<'source>>),
    // mutability, identifier, signature, assignment, expr, semicolon
    Assignment(
        Token,
        Box<Expr<'source>>,
        Option<Box<Expr<'source>>>,
        Token,
        Box<Expr<'source>>,
        Option<Token>,
    ),
    // identifier, asop, expr, semicolon
    Reassignment(Box<Expr<'source>>, Token, Box<Expr<'source>>, Option<Token>),
    // left expr, op, right expr
    BinOp(Box<Expr<'source>>, Token, Box<Expr<'source>>),
    UnaryOp(Box<Expr<'source>>, Token),
    Identity(Lexeme<'source>),
    RetFn(Option<Box<Expr<'source>>>, bool),
    Number(Lexeme<'source>),
    Single(Token),
}

#[macro_export]
macro_rules! some_expr {
    ($val:ident, $($inner:tt)*) => {
        Some(Box::new(Expr::$val($($inner)*)));
    };
}

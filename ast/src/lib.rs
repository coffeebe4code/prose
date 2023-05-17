use lexer::Lexeme;

#[derive(Debug, PartialEq)]
pub enum Expr<'s> {
    //    Body(Vec<Expr<'s>>),
    //    // mutability, identifier, signature, assignment, expr, semicolon
    //    Assignment(
    //        Token,
    //        Box<Expr<'s>>,
    //        Option<Box<Expr<'s>>>,
    //        Token,
    //        Box<Expr<'s>>,
    //        Option<Token>,
    //    ),
    //    // identifier, asop, expr, semicolon
    //    Reassignment(Box<Expr<'s>>, Token, Box<Expr<'s>>, Option<Token>),
    //    // left expr, op, right expr
    BinOp(Box<Expr<'s>>, Lexeme<'s>, Box<Expr<'s>>),
    UnaryOp(Box<Expr<'s>>, Lexeme<'s>),
    //    Identity(Lexeme<'s>),
    RetFn(Box<Expr<'s>>),
    Number(Lexeme<'s>),
    //    Single(Token),
}

#[macro_export]
macro_rules! make_expr {
    ($val:ident, $($inner:tt)*) => {
        Box::new(Expr::$val($($inner)*));
    };
}

#[macro_export]
macro_rules! bubble_expr {
    ($val:ident, $($inner:tt)*) => {
        Some(Ok(Box::new(Expr::$val($($inner)*))));
    };
}

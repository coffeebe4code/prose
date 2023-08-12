use lexer::Lexeme;

#[derive(Debug, Clone, PartialEq)]
pub struct UnOp {
    pub op: Lexeme,
    pub val: Box<Expr>,
}
impl UnOp {
    pub fn new(op: Lexeme, val: Box<Expr>) -> Self {
        UnOp { op, val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub val: Lexeme,
}

impl Symbol {
    pub fn new(val: Lexeme) -> Self {
        Symbol { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    pub val: Lexeme,
}

impl Number {
    pub fn new(val: Lexeme) -> Self {
        Number { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinOp {
    pub left: Box<Expr>,
    pub op: Lexeme,
    pub right: Box<Expr>,
}

impl BinOp {
    pub fn new(left: Box<Expr>, op: Lexeme, right: Box<Expr>) -> Self {
        BinOp { left, op, right }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    BinOp(BinOp),
    UnOp(UnOp),
    Number(Number),
    Symbol(Symbol),
}

#[macro_export]
macro_rules! expr {
    ($val:ident, $($inner:tt)*) => {
        Box::new(Expr::$val($val::new($($inner)*)))
    };
}

#[macro_export]
macro_rules! bubble_expr {
    ($val:ident, $($inner:tt)*) => {
        Ok(Box::new(Expr::$val($val::new($($inner)*))))
    };
}

#[macro_export]
macro_rules! opt_expr {
    ($val:ident, $($inner:tt)*) => {
        Some(Box::new(Expr::$val($val::new($($inner)*))))
    };
}

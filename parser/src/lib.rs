use ast::Expr;
use lexer::ProseLexer;
use tokens::Token;

pub struct Parser<'a> {
    lexer: ProseLexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: ProseLexer<'a>) -> Self {
        Parser { lexer }
    }
    pub fn ident(&mut self) -> Option<Box<Expr<'a>>> {
        if self.lexer.peek()?.is_kind(Token::Symbol) {
            let lexeme = self.lexer.collect().expect("error");
            let expr = Box::new(Expr::Identity(lexeme));
            return Some(expr);
        }
        return None;
    }
    pub fn term(&mut self) -> Option<Box<Expr<'a>>> {
        let mut expr: Option<Box<Expr<'a>>>;
        expr = self.num();
        if expr.is_none() {
            expr = self.ident();
        }
        return expr;
    }
    pub fn low_bin(&mut self) -> Option<Box<Expr<'a>>> {
        let mut left = self.term();
        if left.is_some() {
            if self.lexer.peek()?.is_of_kind(&[Token::Plus, Token::Sub]) {
                let bin = self.lexer.collect().expect("error").token;
                let right = self.term();
                if right.is_none() {
                    return right;
                }
                left = Some(Box::new(Expr::BinOp(
                    left.expect("error"),
                    bin,
                    right.expect("error"),
                )));
            }
        }
        return left;
    }
    pub fn high_bin(&mut self) -> Option<Box<Expr<'a>>> {
        let mut left = self.term();
        if left.is_some() {
            if self
                .lexer
                .peek()?
                .is_of_kind(&[Token::Div, Token::Mul, Token::Mod])
            {
                let bin = self.lexer.collect().expect("error").token;
                let right = self.term();
                if right.is_none() {
                    return right;
                }
                left = Some(Box::new(Expr::BinOp(
                    left.expect("error"),
                    bin,
                    right.expect("error"),
                )));
            }
        }
        return left;
    }
    pub fn num(&mut self) -> Option<Box<Expr<'a>>> {
        if self.lexer.peek()? == Token::Num {
            let lexeme = self.lexer.collect().expect("error");
            let expr = Expr::Number(lexeme);
            return Some(Box::new(expr));
        }
        return None;
    }
    pub fn unary(&mut self) -> Option<Box<Expr<'a>>> {
        let token = self.lexer.peek()?;
        let mut expr: Option<Box<Expr<'a>>> = None;
        if token == Token::Not || token == Token::Sub {
            self.lexer.collect().expect("error");
            expr = self.ident();
            if expr.is_none() {
                expr = self.unary();
            }
            match expr {
                Some(val) => {
                    expr = Some(Box::new(Expr::UnaryOp(val, token)));
                }
                None => (),
            }
        }
        return expr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexeme::Lexeme;
    #[test]
    fn it_should_parse_unary() {
        let lexer = ProseLexer::new("-x");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let test = Box::new(Expr::UnaryOp(
            Box::new(Expr::Identity(Lexeme {
                token: Token::Symbol,
                contents: "y",
            })),
            Token::Sub,
        ));
        assert_eq!(result.unwrap(), test);
    }
}

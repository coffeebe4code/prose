use ast::Expr;
use lexer::ProseLexer;
use tokens::Token;

pub struct Parser<'a> {
    lexer: ProseLexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn ident(&mut self) -> Option<Box<Expr<'a>>> {
        if self.lexer.peek()? == Token::Symbol {
            let lexeme = self.lexer.collect().expect("error");
            let expr = Box::new(Expr::Symbol(lexeme));
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
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

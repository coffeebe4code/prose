use ast::*;
use lexer::ProseLexer;
use token::Token;

pub struct Parser<'a> {
    lexer: ProseLexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: ProseLexer<'a>) -> Self {
        Parser { lexer }
    }
    pub fn ident(&mut self) -> Option<Box<Expr<'a>>> {
        if self.lexer.peek()?.is_kind(Token::Symbol) {
            let lexeme = self.lexer.collect().unwrap();
            let expr = some_expr!(Identity, lexeme);
            return expr;
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
                let bin = self.lexer.collect().unwrap().token;
                let right = self.term();
                if right.is_none() {
                    return right;
                }
                left = some_expr!(BinOp, left.unwrap(), bin, right.unwrap());
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
                let bin = self.lexer.collect().unwrap().token;
                let right = self.term();
                if right.is_none() {
                    return right;
                }
                left = some_expr!(BinOp, left.unwrap(), bin, right.unwrap());
            }
        }
        return left;
    }
    pub fn num(&mut self) -> Option<Box<Expr<'a>>> {
        if self.lexer.peek()? == Token::Num {
            let lexeme = self.lexer.collect().unwrap();
            let expr = some_expr!(Number, lexeme);
            return expr;
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
                    expr = some_expr!(UnaryOp, val, token);
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
                contents: "x",
            })),
            Token::Sub,
        ));
        assert_eq!(result.unwrap(), test);
    }
}

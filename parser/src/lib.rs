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
        let lexeme = self.lexer.collect_if(Token::Symbol)?;
        return some_expr!(Identity, lexeme);
    }
    pub fn term(&mut self) -> Option<Box<Expr<'a>>> {
        let mut expr: Option<Box<Expr<'a>>>;
        expr = self.num();
        if expr.is_none() {
            expr = self.ident();
        }
        return expr;
    }
    pub fn parse_true(&mut self) -> Option<Box<Expr<'a>>> {
        let lexeme = self.lexer.collect_if(Token::True)?;
        return some_expr!(Single, lexeme.token);
    }
    pub fn parse_false(&mut self) -> Option<Box<Expr<'a>>> {
        let lexeme = self.lexer.collect_if(Token::False)?;
        return some_expr!(Single, lexeme.token);
    }
    pub fn parse_null(&mut self) -> Option<Box<Expr<'a>>> {
        let lexeme = self.lexer.collect_if(Token::Null)?;
        return some_expr!(Single, lexeme.token);
    }
    pub fn low_bin(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.term()?;
        let bin = self.lexer.collect_of_if(&[Token::Plus, Token::Sub])?.token;
        let right = self.term()?;
        return some_expr!(BinOp, left, bin, right);
    }
    pub fn high_bin(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.term()?;
        let bin = self
            .lexer
            .collect_of_if(&[Token::Div, Token::Mul, Token::Mod])?
            .token;
        let right = self.term()?;
        return some_expr!(BinOp, left, bin, right);
    }
    pub fn num(&mut self) -> Option<Box<Expr<'a>>> {
        let lexeme = self.lexer.collect_if(Token::Num)?;
        return some_expr!(Number, lexeme);
    }
    pub fn unary(&mut self) -> Option<Box<Expr<'a>>> {
        let token = self.lexer.collect_of_if(&[Token::Not, Token::Sub])?.token;
        let mut expr = self.ident();
        if expr.is_none() {
            expr = self.unary();
        }
        return some_expr!(UnaryOp, expr?, token);
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

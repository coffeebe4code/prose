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
        expr = self.parse_true();
        if expr.is_none() {
            expr = self.parse_false();
        }
        if expr.is_none() {
            expr = self.parse_null();
        }
        if expr.is_none() {
            expr = self.num();
        }
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
    pub fn val_type(&mut self) -> Option<Box<Expr<'a>>> {
        let lexeme = self.lexer.collect_if(Token::Null)?;
        return some_expr!(Single, lexeme.token);
    }
    pub fn parse_return(&mut self) -> Option<Box<Expr<'a>>> {
        //TODO::reevaluate if need optional no return and how that looks in ebnf.
        self.lexer.collect_if(Token::Return)?;
        let expr = self.or_log();
        let semicolon = self.lexer.collect_if(Token::SColon);
        return some_expr!(RetFn, expr, semicolon.is_some());
    }
    pub fn inner_asgnmt(&mut self) -> Option<Box<Expr<'a>>> {
        let mutability =
            self.lexer.collect_of_if(&[Token::Mut, Token::Const])?.token == Token::Const;
        //TODO:: error here on out if none
        let ident = self.ident()?;
        let colon = self.lexer.collect_if(Token::Colon);
        let mut sig = None;
        if colon.is_some() {
            sig = self.signature();
        }
        let assignment = self.lexer.collect_if(Token::As)?.token;
        let expr = self.or_log()?;
        let semicolon = self.lexer.collect_if(Token::SColon);
        return some_expr!(
            Assignment,
            mutability,
            ident,
            sig,
            assignment,
            expr,
            semicolon.is_some()
        );
    }
    pub fn reassignment(&mut self) -> Option<Box<Expr<'a>>> {
        let ident = self.ident()?;
        let asop = self
            .lexer
            .collect_of_if(&[
                Token::As,
                Token::DivAs,
                Token::SubAs,
                Token::AddAs,
                Token::MulAs,
                Token::AndAs,
                Token::XorAs,
                Token::OrAs,
            ])?
            .token;
        let expr = self.or_log()?;
        let semicolon = self.lexer.collect_if(Token::SColon);
        return some_expr!(Reassignment, ident, asop, expr, semicolon.is_some());
    }
    pub fn signature(&mut self) -> Option<Box<Expr<'a>>> {
        //TODO::Impl
        return None;
    }
    pub fn comp(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.low_bin()?;
        let bin = self
            .lexer
            .collect_of_if(&[Token::Gt, Token::GtEq, Token::Lt, Token::LtEq])?
            .token;
        // TODO:: Error if expr is none
        let right = self.low_bin()?;
        return some_expr!(BinOp, left, bin, right);
    }
    pub fn or_log(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.and_log()?;
        let bin = self.lexer.collect_of_if(&[Token::OrLog])?.token;
        // TODO:: Error if expr is none
        let right = self.and_log()?;
        return some_expr!(BinOp, left, bin, right);
    }
    pub fn and_log(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.equality()?;
        let bin = self.lexer.collect_of_if(&[Token::AndLog])?.token;
        // TODO:: Error if expr is none
        let right = self.equality()?;
        return some_expr!(BinOp, left, bin, right);
    }
    pub fn equality(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.comp()?;
        let bin = self
            .lexer
            .collect_of_if(&[Token::NotEquality, Token::Equality])?
            .token;
        // TODO:: Error if expr is none
        let right = self.comp()?;
        return some_expr!(BinOp, left, bin, right);
    }
    pub fn low_bin(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.term()?;
        let bin = self.lexer.collect_of_if(&[Token::Plus, Token::Sub])?.token;
        // TODO:: Error if expr is none
        let right = self.term()?;
        return some_expr!(BinOp, left, bin, right);
    }
    pub fn high_bin(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.term()?;
        let bin = self
            .lexer
            .collect_of_if(&[Token::Div, Token::Mul, Token::Mod])?
            .token;
        // TODO:: Error if expr is none
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
        // TODO:: Error if expr is none
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

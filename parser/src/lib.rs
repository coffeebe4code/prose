use ast::*;
use lexeme::Grabable;
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
    pub fn inner_assignment(&mut self) -> Option<Box<Expr<'a>>> {
        let mutability = self.lexer.collect_of_if(&[Token::Mut, Token::Const])?.token;
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
            semicolon.maybe_token()
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
        return some_expr!(Reassignment, ident, asop, expr, semicolon.maybe_token());
    }
    pub fn signature(&mut self) -> Option<Box<Expr<'a>>> {
        //TODO::Impl
        return None;
    }
    pub fn or_log(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.and_log()?;
        let bin = self.lexer.collect_of_if(&[Token::OrLog]);
        if let Some(x) = bin {
            // TODO:: Error if expr is none
            let right = self.and_log()?;
            return some_expr!(BinOp, left, x.token, right);
        }
        return Some(left);
    }
    pub fn and_log(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.equality()?;
        let bin = self.lexer.collect_of_if(&[Token::AndLog]);
        if let Some(x) = bin {
            // TODO:: Error if expr is none
            let right = self.equality()?;
            return some_expr!(BinOp, left, x.token, right);
        }
        return Some(left);
    }
    pub fn equality(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.comp()?;
        let bin = self
            .lexer
            .collect_of_if(&[Token::NotEquality, Token::Equality]);
        if let Some(x) = bin {
            // TODO:: Error if expr is none
            let right = self.comp()?;
            return some_expr!(BinOp, left, x.token, right);
        }
        return Some(left);
    }
    pub fn comp(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.low_bin()?;
        let bin = self
            .lexer
            .collect_of_if(&[Token::Gt, Token::GtEq, Token::Lt, Token::LtEq]);
        if let Some(x) = bin {
            // TODO:: Error if expr is none
            let right = self.low_bin()?;
            return some_expr!(BinOp, left, x.token, right);
        }
        return Some(left);
    }
    pub fn low_bin(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.high_bin()?;
        let bin = self.lexer.collect_of_if(&[Token::Plus, Token::Sub]);
        if let Some(x) = bin {
            // TODO:: Error if expr is none
            let right = self.high_bin()?;
            return some_expr!(BinOp, left, x.token, right);
        }
        return Some(left);
    }
    pub fn high_bin(&mut self) -> Option<Box<Expr<'a>>> {
        let left = self.unary()?;
        let bin = self
            .lexer
            .collect_of_if(&[Token::Div, Token::Mul, Token::Mod]);
        if let Some(x) = bin {
            // TODO:: Error if expr is none
            let right = self.unary()?;
            return some_expr!(BinOp, left, x.token, right);
        }
        return Some(left);
    }
    pub fn num(&mut self) -> Option<Box<Expr<'a>>> {
        let lexeme = self.lexer.collect_if(Token::Num)?;
        return some_expr!(Number, lexeme);
    }
    pub fn unary(&mut self) -> Option<Box<Expr<'a>>> {
        let token = self.lexer.collect_of_if(&[Token::Not, Token::Sub]);
        println!("token {:#?}", token);
        if let Some(x) = token {
            let expr = self.unary();
            println!("unary expr {:#?}", expr);
            // TODO:: Error if expr is none
            return some_expr!(UnaryOp, expr.unwrap(), x.token);
        } else {
            let expr = self.term();
            println!("term expr {:#?}", expr);
            return expr;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexeme::Lexeme;
    #[test]
    fn it_should_parse_unary() {
        let lexer = ProseLexer::new("--x");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let first = some_expr!(
            UnaryOp,
            some_expr!(Identity, Lexeme::new("x", Token::Symbol)).unwrap(),
            Token::Sub
        );
        let test = some_expr!(UnaryOp, first.unwrap(), Token::Sub);
        assert_eq!(result.unwrap(), test.unwrap());
    }
    #[test]
    fn it_should_parse_terminal() {
        let lexer = ProseLexer::new("5");
        let mut parser = Parser::new(lexer);
        let result = parser.term();
        let test = some_expr!(Number, Lexeme::new("5", Token::Num));
        assert_eq!(result.unwrap(), test.unwrap());
    }
    #[test]
    fn it_should_parse_low_bin() {
        let lexer = ProseLexer::new("5 - 5");
        let mut parser = Parser::new(lexer);
        let result = parser.low_bin();
        let left = some_expr!(Number, Lexeme::new("5", Token::Num));
        let right = some_expr!(Number, Lexeme::new("5", Token::Num));
        let test = some_expr!(BinOp, left.unwrap(), Token::Sub, right.unwrap());
        assert_eq!(result.unwrap(), test.unwrap());
    }
    #[test]
    fn it_should_parse_inner_assignment() {
        let lexer = ProseLexer::new("const x = 5 + 5;");
        let mut parser = Parser::new(lexer);
        let result = parser.inner_assignment();
        let left = some_expr!(Number, Lexeme::new("5", Token::Num));
        let right = some_expr!(Number, Lexeme::new("5", Token::Num));
        let ident = some_expr!(Identity, Lexeme::new("x", Token::Symbol));
        let mutability = Token::Const;
        let bin = some_expr!(BinOp, left.unwrap(), Token::Plus, right.unwrap());
        let assignment = some_expr!(
            Assignment,
            mutability,
            ident.unwrap(),
            None,
            Token::As,
            bin.unwrap(),
            Some(Token::SColon)
        );
        assert_eq!(result.unwrap(), assignment.unwrap());
    }
    #[test]
    fn it_should_parse_reassignment() {
        let lexer = ProseLexer::new("x += 5");
        let mut parser = Parser::new(lexer);
        let result = parser.reassignment();
        let right = some_expr!(Number, Lexeme::new("5", Token::Num));
        let ident = some_expr!(Identity, Lexeme::new("x", Token::Symbol));
        let assignment = some_expr!(
            Reassignment,
            ident.unwrap(),
            Token::AddAs,
            right.unwrap(),
            None
        );
        assert_eq!(result.unwrap(), assignment.unwrap());
    }
}

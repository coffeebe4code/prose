use ast::*;
use lexer::ProseLexer;
use perror::*;
use token::Token;

type ResultExpr<'a> = Result<Box<Expr<'a>>>;
type BubbleExpr<'a> = Option<ResultExpr<'a>>;

pub struct Parser<'a> {
    lexer: ProseLexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: ProseLexer<'a>) -> Self {
        Parser { lexer }
    }
    // pub fn ident(&mut self) -> Result<Box<Expr<'a>>, Error> {
    //     let lexeme = self.lexer.collect_if(Token::Symbol);
    //     return Ok(make_expr!(Identity, lexeme));
    // }
    // pub fn term(&mut self) -> Option<Box<Expr<'a>>> {
    //     let mut expr: Option<Box<Expr<'a>>>;
    //     expr = self.parse_true();
    //     if expr.is_none() {
    //         expr = self.parse_false();
    //     }
    //     if expr.is_none() {
    //         expr = self.parse_null();
    //     }
    //     if expr.is_none() {
    //         expr = self.num();
    //     }
    //     if expr.is_none() {
    //         expr = self.ident();
    //     }
    //     return expr;
    // }
    // pub fn parse_true(&mut self) -> Option<Box<Expr<'a>>> {
    //     let lexeme = self.lexer.collect_if(Token::True)?;
    //     return some_expr!(Single, lexeme.token);
    // }
    // pub fn parse_false(&mut self) -> Option<Box<Expr<'a>>> {
    //     let lexeme = self.lexer.collect_if(Token::False)?;
    //     return some_expr!(Single, lexeme.token);
    // }
    // pub fn parse_null(&mut self) -> Option<Box<Expr<'a>>> {
    //     let lexeme = self.lexer.collect_if(Token::Null)?;
    //     return some_expr!(Single, lexeme.token);
    // }
    // pub fn val_type(&mut self) -> Option<Box<Expr<'a>>> {
    //     let lexeme = self.lexer.collect_if(Token::Null)?;
    //     return some_expr!(Single, lexeme.token);
    // }
    // pub fn parse_return(&mut self) -> Option<Box<Expr<'a>>> {
    //     //TODO::reevaluate if need optional no return and how that looks in ebnf.
    //     self.lexer.collect_if(Token::Return)?;
    //     let expr = self.or_log();
    //     let semicolon = self.lexer.collect_if(Token::SColon);
    //     return some_expr!(RetFn, expr, semicolon.is_some());
    // }
    // pub fn inner_assignment(&mut self) -> Option<Box<Expr<'a>>> {
    //     let mutability = self.lexer.collect_of_if(&[Token::Mut, Token::Const])?.token;
    //     //TODO:: error here on out if none
    //     let ident = self.ident()?;
    //     let colon = self.lexer.collect_if(Token::Colon);
    //     let mut sig = None;
    //     if colon.is_some() {
    //         sig = self.signature();
    //     }
    //     let assignment = self.lexer.collect_if(Token::As)?.token;
    //     let expr = self.or_log()?;
    //     let semicolon = self.lexer.collect_if(Token::SColon);
    //     return some_expr!(
    //         Assignment,
    //         mutability,
    //         ident,
    //         sig,
    //         assignment,
    //         expr,
    //         semicolon.maybe_token()
    //     );
    // }
    // pub fn reassignment(&mut self) -> Option<Box<Expr<'a>>> {
    //     let ident = self.ident()?;
    //     let asop = self
    //         .lexer
    //         .collect_of_if(&[
    //             Token::As,
    //             Token::DivAs,
    //             Token::SubAs,
    //             Token::AddAs,
    //             Token::MulAs,
    //             Token::AndAs,
    //             Token::XorAs,
    //             Token::OrAs,
    //         ])?
    //         .token;
    //     let expr = self.or_log()?;
    //     let semicolon = self.lexer.collect_if(Token::SColon);
    //     return some_expr!(Reassignment, ident, asop, expr, semicolon.maybe_token());
    // }
    // pub fn signature(&mut self) -> Option<Box<Expr<'a>>> {
    //     //TODO::Impl
    //     return None;
    // }
    // pub fn or_log(&mut self) -> Option<Box<Expr<'a>>> {
    //     let left = self.and_log()?;
    //     let bin = self.lexer.collect_of_if(&[Token::OrLog]);
    //     if let Some(x) = bin {
    //         // TODO:: Error if expr is none
    //         let right = self.and_log()?;
    //         return some_expr!(BinOp, left, x.token, right);
    //     }
    //     return Some(left);
    // }
    // pub fn and_log(&mut self) -> Option<Box<Expr<'a>>> {
    //     let left = self.equality()?;
    //     let bin = self.lexer.collect_of_if(&[Token::AndLog]);
    //     if let Some(x) = bin {
    //         // TODO:: Error if expr is none
    //         let right = self.equality()?;
    //         return some_expr!(BinOp, left, x.token, right);
    //     }
    //     return Some(left);
    // }
    // pub fn equality(&mut self) -> Option<Box<Expr<'a>>> {
    //     let left = self.comp()?;
    //     let bin = self
    //         .lexer
    //         .collect_of_if(&[Token::NotEquality, Token::Equality]);
    //     if let Some(x) = bin {
    //         // TODO:: Error if expr is none
    //         let right = self.comp()?;
    //         return some_expr!(BinOp, left, x.token, right);
    //     }
    //     return Some(left);
    // }
    // pub fn comp(&mut self) -> Option<Box<Expr<'a>>> {
    //     let left = self.low_bin()?;
    //     let bin = self
    //         .lexer
    //         .collect_of_if(&[Token::Gt, Token::GtEq, Token::Lt, Token::LtEq]);
    //     if let Some(x) = bin {
    //         // TODO:: Error if expr is none
    //         let right = self.low_bin()?;
    //         return some_expr!(BinOp, left, x.token, right);
    //     }
    //     return Some(left);
    // }
    //pub fn low_bin(&mut self) -> Option<Result<Box<Expr<'a>>>> {
    //    let left = self.high_bin()?;
    //    let bin = self.lexer.collect_of_if(&[Token::Plus, Token::Sub]);
    //    if let Some(x) = bin {
    //        // TODO:: Error if expr is none
    //        let right = self.high_bin()?;
    //        return some_expr!(BinOp, left, x, right);
    //    }
    //    return Some(Ok(left));
    //}
    //pub fn high_bin(&mut self) -> Option<Result<Box<Expr<'a>>>> {
    //    let left = self.unary()?.bubble_error(|x| { });
    //    if let Ok(a) = left {
    //        let bin = self
    //            .lexer
    //            .collect_of_if(&[Token::Div, Token::Mul, Token::Mod]);
    //        if let Some(x) = bin {
    //            let right = self.unary();
    //            match right {
    //                None => {
    //                    return Some(Err(ParseError::new("expected unary or num", 0..2)));
    //                }
    //                Some(val) {
    //                    match val {
    //                        Err(t) => return Some(Err(t));

    //                    }

    //                }
    //            }
    //            if let None = right {
    //                return Some(Err(ParseError::new("expected unary or num", 0..2)));
    //            }
    //            if let None = right {
    //                return Some(Err(ParseError::new("expected unary or num", 0..2)));
    //            }
    //            return some_expr!(BinOp, a, x, right);
    //        }
    //    }
    //    return Some(left);
    //}
    pub fn num(&mut self) -> BubbleExpr<'a> {
        let lexeme = self.lexer.collect_if(Token::Num)?;
        return bubble_expr!(Number, lexeme);
    }
    pub fn unary(&mut self) -> Option<Result<Box<Expr<'a>>>> {
        let lexeme = self.lexer.collect_of_if(&[Token::Not, Token::Sub]);
        if let Some(x) = lexeme {
            let expr = self.unary().expect_some_val("Error");
            return expr.bubble_error(|result| bubble_expr!(UnaryOp, result, x));
        }
        self.num()
    }
}

trait ExpectSomeVal<'a> {
    fn expect_some_val(self, title: &'static str) -> BubbleExpr<'a>;
}

trait BubbleError<'a> {
    fn bubble_error(self, func: impl FnOnce(Box<Expr<'a>>) -> BubbleExpr<'a>) -> BubbleExpr<'a>;
}

impl<'a> ExpectSomeVal<'a> for BubbleExpr<'a> {
    fn expect_some_val(self, title: &'static str) -> Option<Result<Box<Expr<'a>>>> {
        if self.is_none() {
            return Some(Err(ParseError::new(title)));
        }
        self
    }
}

impl<'a> BubbleError<'a> for BubbleExpr<'a> {
    fn bubble_error(self, func: impl FnOnce(Box<Expr<'a>>) -> BubbleExpr<'a>) -> BubbleExpr<'a> {
        match self {
            None => return None,
            Some(val) => match val {
                Err(err) => {
                    return Some(Err(err));
                }
                Ok(inner) => {
                    return func(inner);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexeme;
    #[test]
    fn it_should_parse_unary() {
        let lexer = ProseLexer::new("-5");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let first = make_expr!(
            UnaryOp,
            make_expr!(
                Number,
                Lexeme {
                    slice: "5",
                    token: Token::Num,
                    span: 1..2
                }
            ),
            Lexeme {
                slice: "-",
                token: Token::Sub,
                span: 0..1
            }
        );
        assert_eq!(result.unwrap().unwrap(), first);
    }
}

use ast::*;
use lexer::ProseLexer;
use perror::*;
use token::Token;

pub type ResultExpr = Result<Box<Expr>>;
pub type BubbleExpr = Option<ResultExpr>;
pub type OptExpr = Option<Box<Expr>>;

pub struct Parser<'s> {
    lexer: ProseLexer<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(lexer: ProseLexer<'s>) -> Self {
        Parser { lexer }
    }
    pub fn or_cmp(&mut self) -> ResultExpr {
        self.and_cmp().bubble_error(|mut left| {
            while let Some(bin) = self.lexer.collect_if(Token::Or) {
                left = self
                    .and_cmp()
                    .bubble_error(|right| bubble_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn and_cmp(&mut self) -> ResultExpr {
        self.equality().bubble_error(|mut left| {
            while let Some(bin) = self.lexer.collect_if(Token::And) {
                left = self
                    .equality()
                    .bubble_error(|right| bubble_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn equality(&mut self) -> ResultExpr {
        self.cmp().bubble_error(|mut left| {
            while let Some(bin) = self
                .lexer
                .collect_of_if(&[Token::Equality, Token::NotEquality])
            {
                left = self
                    .cmp()
                    .bubble_error(|right| bubble_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn cmp(&mut self) -> ResultExpr {
        self.low_bin().bubble_error(|mut left| {
            while let Some(bin) =
                self.lexer
                    .collect_of_if(&[Token::Gt, Token::GtEq, Token::Lt, Token::LtEq])
            {
                left = self
                    .low_bin()
                    .bubble_error(|right| bubble_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn low_bin(&mut self) -> ResultExpr {
        self.high_bin().bubble_error(|mut left| {
            while let Some(bin) = self.lexer.collect_of_if(&[Token::Plus, Token::Sub]) {
                left = self
                    .high_bin()
                    .bubble_error(|right| bubble_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn high_bin(&mut self) -> ResultExpr {
        self.unary().bubble_error(|mut left| {
            while let Some(bin) = self
                .lexer
                .collect_of_if(&[Token::Div, Token::Mul, Token::Mod])
            {
                left = self
                    .unary()
                    .bubble_error(|right| bubble_expr!(BinOp, left, bin, right))?
            }
            return Ok(left);
        })
    }
    pub fn unary(&mut self) -> ResultExpr {
        let lexeme = self.lexer.collect_of_if(&[Token::Not, Token::Sub]);
        if let Some(x) = lexeme {
            let expr = self.unary();
            return expr.bubble_error(|result| bubble_expr!(UnOp, x, result));
        }
        self.num()
            .or_else_do(|| self.ident())
            .convert("number or identifier".to_string())
    }
    pub fn num(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::Num)?;
        opt_expr!(Number, lexeme)
    }
    pub fn ident(&mut self) -> OptExpr {
        let lexeme = self.lexer.collect_if(Token::Symbol)?;
        opt_expr!(Symbol, lexeme)
    }
}

trait ExpectSomeVal {
    fn expect_some_val(self, title: String) -> BubbleExpr;
}

trait BubbleError {
    fn bubble_error(self, func: impl FnOnce(Box<Expr>) -> ResultExpr) -> ResultExpr;
}

trait OrElseDo {
    fn or_else_do(self, func: impl FnOnce() -> OptExpr) -> OptExpr;
}

trait Convert {
    fn convert(self, title: String) -> ResultExpr;
}

trait ChainExpect {
    fn chain_expect(self, title: String) -> ResultExpr;
}

impl ExpectSomeVal for BubbleExpr {
    fn expect_some_val(self, title: String) -> BubbleExpr {
        if self.is_none() {
            return Some(Err(ParseError::new(title)));
        }
        self
    }
}

impl BubbleError for ResultExpr {
    fn bubble_error(self, func: impl FnOnce(Box<Expr>) -> ResultExpr) -> ResultExpr {
        match self {
            Err(err) => Err(err),
            Ok(inner) => func(inner),
        }
    }
}

impl OrElseDo for OptExpr {
    fn or_else_do(self, func: impl FnOnce() -> OptExpr) -> OptExpr {
        match self {
            None => return func(),
            Some(val) => return Some(val),
        }
    }
}

impl Convert for OptExpr {
    fn convert(self, title: String) -> ResultExpr {
        match self {
            None => Err(ParseError::new(title)),
            Some(val) => Ok(val),
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
        let first = UnOp::new(
            Lexeme {
                slice: String::from("-"),
                token: Token::Sub,
                span: 0..1,
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("5"),
                    token: Token::Num,
                    span: 1..2,
                }
            ),
        );
        assert_eq!(result.unwrap(), Box::new(Expr::UnOp(first)));
    }
    #[test]
    fn it_should_parse_unary_num() {
        let lexer = ProseLexer::new("5");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let first = Number::new(Lexeme {
            slice: String::from("5"),
            token: Token::Num,
            span: 0..1,
        });
        assert_eq!(result.unwrap(), Box::new(Expr::Number(first)));
    }
    #[test]
    fn it_should_error_unary() {
        let lexer = ProseLexer::new("-");
        let mut parser = Parser::new(lexer);
        let result = parser.unary();
        let error = ParseError::new("number or identifier".to_string());
        assert_eq!(result.expect_err("failed test"), error);
    }
    #[test]
    fn it_should_error_high_bin() {
        let lexer = ProseLexer::new("5 *");
        let mut parser = Parser::new(lexer);
        let result = parser.high_bin();
        let error = ParseError::new("number or identifier".to_string());
        assert_eq!(result.expect_err("failed test"), error);
    }
    #[test]
    fn it_should_parse_high_bin() {
        let lexer = ProseLexer::new("5 * 2");
        let mut parser = Parser::new(lexer);
        let result = parser.high_bin();
        let expr = BinOp::new(
            expr!(
                Number,
                Lexeme {
                    slice: String::from("5"),
                    token: Token::Num,
                    span: 0..1,
                }
            ),
            Lexeme {
                slice: String::from("*"),
                token: Token::Mul,
                span: 2..3,
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("2"),
                    token: Token::Num,
                    span: 4..5,
                }
            ),
        );
        assert_eq!(result.unwrap(), Box::new(Expr::BinOp(expr)));
    }
    #[test]
    fn it_should_parse_low_bin_mul() {
        let lexer = ProseLexer::new("5 + 3 * 2 + 1");
        let mut parser = Parser::new(lexer);
        let result = parser.low_bin();
        let expr = expr!(
            BinOp,
            expr!(
                BinOp,
                expr!(
                    Number,
                    Lexeme {
                        slice: String::from("5"),
                        token: Token::Num,
                        span: 0..1,
                    }
                ),
                Lexeme {
                    slice: String::from("+"),
                    token: Token::Plus,
                    span: 2..3,
                },
                expr!(
                    BinOp,
                    expr!(
                        Number,
                        Lexeme {
                            slice: String::from("3"),
                            token: Token::Num,
                            span: 4..5
                        }
                    ),
                    Lexeme {
                        slice: String::from("*"),
                        token: Token::Mul,
                        span: 6..7
                    },
                    expr!(
                        Number,
                        Lexeme {
                            slice: String::from("2"),
                            token: Token::Num,
                            span: 8..9
                        }
                    )
                ),
            ),
            Lexeme {
                slice: String::from("+"),
                token: Token::Plus,
                span: 10..11,
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("1"),
                    token: Token::Num,
                    span: 12..13,
                }
            )
        );
        assert_eq!(result.unwrap(), expr);
    }

    #[test]
    fn it_should_parse_low_bin() {
        let lexer = ProseLexer::new("5 + 3 + 2");
        let mut parser = Parser::new(lexer);
        let result = parser.low_bin();
        let expr = expr!(
            BinOp,
            expr!(
                BinOp,
                expr!(
                    Number,
                    Lexeme {
                        slice: String::from("5"),
                        token: Token::Num,
                        span: 0..1,
                    }
                ),
                Lexeme {
                    slice: String::from("+"),
                    token: Token::Plus,
                    span: 2..3,
                },
                expr!(
                    Number,
                    Lexeme {
                        slice: String::from("3"),
                        token: Token::Num,
                        span: 4..5
                    }
                )
            ),
            Lexeme {
                slice: String::from("+"),
                token: Token::Plus,
                span: 6..7
            },
            expr!(
                Number,
                Lexeme {
                    slice: String::from("2"),
                    token: Token::Num,
                    span: 8..9
                }
            )
        );
        assert_eq!(result.unwrap(), expr);
    }
}

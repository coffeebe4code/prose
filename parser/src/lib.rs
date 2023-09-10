use ast::*;
use lexer::Lexeme;
use lexer::ProseLexer;
use perror::*;
use token::Token;

pub type ResultOptExpr = Result<Option<Box<Expr>>>;
pub type ResultExpr = Result<Box<Expr>>;
pub type OptExpr = Option<Box<Expr>>;

pub struct Parser<'s> {
    lexer: ProseLexer<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(lexer: ProseLexer<'s>) -> Self {
        Parser { lexer }
    }
    pub fn ret(&mut self) -> ResultExpr {
        let span = self
            .lexer
            .collect_if(Token::Return)
            .expect_token("expected return keyword".to_string())?;
        self.or_cmp().result_or(|expr| {
            self.lexer
                .collect_if(Token::SColon)
                .expect_token("expected ';'".to_string())?;
            result_expr!(RetOp, span, expr)
        })
    }
    pub fn func(&mut self) -> ResultExpr {
        let has_pub = self.lexer.collect_if(Token::Pub);
        let mutability = self
            .lexer
            .collect_of_if(&[Token::Let, Token::Const])
            .expect_token("expected mutability".to_string())?;
        let identifier = self
            .ident()
            .expect_expr("expected identifier".to_string())?;
        let _ = self
            .lexer
            .collect_if(Token::As)
            .expect_token("expected =".to_string())?;
        let _ = self
            .lexer
            .collect_if(Token::Func)
            .expect_token("expected fn keyword".to_string())?;
        let _ = self
            .lexer
            .collect_if(Token::OParen)
            .expect_token("expected '('".to_string())?;
        let args = self.args()?;
        let _ = self
            .lexer
            .collect_if(Token::CParen)
            .expect_token("expected '('".to_string())?;
        let block = self.block()?;
        result_expr!(FuncDef, has_pub, mutability, identifier, args, block)
    }
    pub fn ty(&mut self) -> ResultExpr {
        self.lexer
            .collect_of_if(&[Token::Num, Token::Any, Token::U64])
            .convert_expr(|span| expr!(TypeSimple, span))
            .convert_to_result("expected type".to_string())
    }
    pub fn args(&mut self) -> Result<Option<Vec<Box<Expr>>>> {
        if let Some(arg_local) = self.arg() {
            let mut arg_list: Vec<Box<Expr>> = vec![];
            arg_list.push(arg_local);
            while let Some(_comma) = self.lexer.collect_if(Token::Comma) {
                arg_list.push(
                    self.arg()
                        .expect_expr("expected argument definition".to_string())?,
                );
            }
            return Ok(Some(arg_list));
        }
        Ok(None)
    }
    pub fn arg(&mut self) -> OptExpr {
        self.ident()
    }
    pub fn block(&mut self) -> ResultExpr {
        self.lexer
            .collect_if(Token::OBrace)
            .expect_token("expected '{'".to_string())?;
        self.ret().result_or(|expr| {
            self.lexer
                .collect_if(Token::CBrace)
                .expect_token("expected '}'".to_string())?;
            result_expr!(Block, vec![expr])
        })
    }
    pub fn or_cmp(&mut self) -> ResultExpr {
        self.and_cmp().result_or(|mut left| {
            while let Some(bin) = self.lexer.collect_if(Token::Or) {
                left = self
                    .and_cmp()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn and_cmp(&mut self) -> ResultExpr {
        self.equality().result_or(|mut left| {
            while let Some(bin) = self.lexer.collect_if(Token::And) {
                left = self
                    .equality()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn equality(&mut self) -> ResultExpr {
        self.cmp().result_or(|mut left| {
            while let Some(bin) = self
                .lexer
                .collect_of_if(&[Token::Equality, Token::NotEquality])
            {
                left = self
                    .cmp()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn cmp(&mut self) -> ResultExpr {
        self.low_bin().result_or(|mut left| {
            while let Some(bin) =
                self.lexer
                    .collect_of_if(&[Token::Gt, Token::GtEq, Token::Lt, Token::LtEq])
            {
                left = self
                    .low_bin()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn low_bin(&mut self) -> ResultExpr {
        self.high_bin().result_or(|mut left| {
            while let Some(bin) = self.lexer.collect_of_if(&[Token::Plus, Token::Sub]) {
                left = self
                    .high_bin()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            Ok(left)
        })
    }
    pub fn high_bin(&mut self) -> ResultExpr {
        self.unary().result_or(|mut left| {
            while let Some(bin) = self
                .lexer
                .collect_of_if(&[Token::Div, Token::Mul, Token::Mod])
            {
                left = self
                    .unary()
                    .result_or(|right| result_expr!(BinOp, left, bin, right))?
            }
            return Ok(left);
        })
    }
    pub fn unary(&mut self) -> ResultExpr {
        let lexeme = self.lexer.collect_of_if(&[Token::Not, Token::Sub]);
        if let Some(x) = lexeme {
            let expr = self.unary();
            return expr.result_or(|result| result_expr!(UnOp, x, result));
        }
        self.num()
            .if_none_do(|| self.ident())
            .convert_to_result("number or identifier".to_string())
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

trait ExpectToken {
    fn expect_token(self, title: String) -> Result<Lexeme>;
}

trait ExpectExpr {
    fn expect_expr(self, title: String) -> ResultExpr;
}

trait ResultOr {
    fn result_or(self, func: impl FnOnce(Box<Expr>) -> ResultExpr) -> ResultExpr;
}

trait ResultOptOr {
    fn result_opt_or(self, func: impl FnOnce(Box<Expr>) -> ResultOptExpr) -> ResultOptExpr;
}

trait IfNoneDo {
    fn if_none_do(self, func: impl FnOnce() -> OptExpr) -> OptExpr;
}

trait ConvertToResult {
    fn convert_to_result(self, title: String) -> ResultExpr;
}

trait ConvertOptExpr {
    fn convert_expr(self, func: impl FnOnce(Lexeme) -> Box<Expr>) -> OptExpr;
}

trait ChainExpect {
    fn chain_expect(self, title: String) -> ResultExpr;
}

impl ResultOr for ResultExpr {
    fn result_or(self, func: impl FnOnce(Box<Expr>) -> ResultExpr) -> ResultExpr {
        match self {
            Err(err) => Err(err),
            Ok(inner) => func(inner),
        }
    }
}

impl IfNoneDo for OptExpr {
    fn if_none_do(self, func: impl FnOnce() -> OptExpr) -> OptExpr {
        match self {
            None => return func(),
            Some(val) => return Some(val),
        }
    }
}

impl ConvertOptExpr for Option<Lexeme> {
    fn convert_expr(self, func: impl FnOnce(Lexeme) -> Box<Expr>) -> OptExpr {
        match self {
            None => None,
            Some(val) => Some(func(val)),
        }
    }
}

impl ExpectToken for Option<Lexeme> {
    fn expect_token(self, title: String) -> Result<Lexeme> {
        match self {
            None => Err(ParserError::new(title)),
            Some(val) => Ok(val),
        }
    }
}

impl ExpectExpr for OptExpr {
    fn expect_expr(self, title: String) -> ResultExpr {
        match self {
            None => Err(ParserError::new(title)),
            Some(val) => Ok(val),
        }
    }
}

impl ConvertToResult for OptExpr {
    fn convert_to_result(self, title: String) -> ResultExpr {
        match self {
            None => Err(ParserError::new(title)),
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
        let error = ParserError::new("number or identifier".to_string());
        assert_eq!(result.expect_err("failed test"), error);
    }
    #[test]
    fn it_should_error_high_bin() {
        let lexer = ProseLexer::new("5 *");
        let mut parser = Parser::new(lexer);
        let result = parser.high_bin();
        let error = ParserError::new("number or identifier".to_string());
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
    #[test]
    fn it_should_parse_fn() {
        let lexer = ProseLexer::new("pub const add = fn(x) { return x; }");
        let mut parser = Parser::new(lexer);
        let result = parser.func();
        let expr = expr!(
            FuncDef,
            Some(Lexeme {
                slice: String::from("pub"),
                token: Token::Pub,
                span: 0..3
            }),
            Lexeme {
                slice: "const".to_string(),
                token: Token::Const,
                span: 4..9
            },
            expr!(
                Symbol,
                Lexeme {
                    slice: "add".to_string(),
                    token: Token::Symbol,
                    span: 10..13
                }
            ),
            Some(vec![expr!(
                Symbol,
                Lexeme {
                    slice: "x".to_string(),
                    token: Token::Symbol,
                    span: 19..20
                }
            )]),
            expr!(
                Block,
                vec![expr!(
                    RetOp,
                    Lexeme {
                        slice: "return".to_string(),
                        token: Token::Return,
                        span: 24..30
                    },
                    expr!(
                        Symbol,
                        Lexeme {
                            slice: "x".to_string(),
                            token: Token::Symbol,
                            span: 31..32
                        }
                    )
                )]
            )
        );
        assert_eq!(result.unwrap(), expr);
    }
}

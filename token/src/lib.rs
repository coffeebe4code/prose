use logos::Logos;

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
pub enum Token {
    #[token("import")]
    Import,
    #[token("define")]
    Define,
    #[token("macro")]
    Macro,
    #[token("test")]
    Test,
    #[token("bench")]
    Bench,
    #[token("release")]
    Release,
    #[token("debug")]
    Debug,
    #[token("mut")]
    Mut,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("once")]
    Once,
    #[token("i32")]
    I32,
    #[token("u32")]
    U32,
    #[token("i64")]
    I64,
    #[token("u64")]
    U64,
    #[token("i16")]
    I16,
    #[token("u16")]
    U16,
    #[token("u8")]
    U8,
    #[token("i8")]
    I8,
    #[token("bit")]
    Bit,
    #[token("f64")]
    F64,
    #[token("f32")]
    F32,
    #[token("d32")]
    D32,
    #[token("d64")]
    D64,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("type")]
    Type,
    #[token("this")]
    This,
    #[token("self")]
    WSelf,
    #[token("null")]
    Null,
    #[token("undef")]
    Undef,
    #[token("char")]
    Char,
    #[token("string")]
    WString,
    #[token("inline")]
    Inline,
    #[token("static")]
    Static,
    #[token("switch")]
    Switch,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("of")]
    Of,
    #[token("break")]
    Break,
    #[token("enum")]
    Enum,
    #[token("pub")]
    Pub,
    #[token("return")]
    Return,
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[token("box")]
    WBox,
    #[token("trait")]
    Trait,
    #[token("ptr")]
    Ptr,
    #[token("match")]
    Match,
    #[token("addr")]
    Addr,
    #[token("vol")]
    Vol,
    #[token("list")]
    List,
    #[token("arr")]
    Arr,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("void")]
    Void,
    #[token("interface")]
    Interface,
    #[token("generic")]
    Generic,
    #[token("never")]
    Never,
    #[token("bool")]
    Bool,
    #[token("byte")]
    Byte,
    #[token("contract")]
    Contract,
    #[token("fn")]
    Fun,
    #[token("func")]
    Func,

    #[token("->")]
    Yield,
    #[token("=>")]
    Lambda,
    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
    #[token("{")]
    OBrace,
    #[token("}")]
    CBrace,
    #[token("[")]
    OArray,
    #[token("]")]
    CArray,

    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("$")]
    Dollar,
    #[token("?")]
    Question,
    #[token("#")]
    Pound,
    #[token(":")]
    Colon,
    #[token(";")]
    SColon,
    #[token("`")]
    Backtick,
    #[token("@")]
    At,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    #[token("/")]
    Div,
    #[token("\\")]
    BSlash,
    #[token("+")]
    Plus,
    #[token("_")]
    Rest,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("|")]
    Or,
    #[token("&")]
    And,
    #[token("^")]
    Xor,
    #[token("<<")]
    LShift,
    #[token(">>")]
    RShift,
    #[token("~")]
    Not,
    #[token("=")]
    As,
    #[token("~=")]
    NotAs,
    #[token("|=")]
    OrAs,
    #[token("^=")]
    XorAs,
    #[token("<<=")]
    LShiftAs,
    #[token(">>=")]
    RShiftAs,
    #[token("&&")]
    AndLog,
    #[token("||")]
    OrLog,
    #[token("!=")]
    NotEquality,
    #[token("==")]
    Equality,
    #[token("!")]
    NotLog,
    #[token("%")]
    Mod,
    #[token("++")]
    Inc,
    #[token("--")]
    Dec,
    #[token("+=")]
    AddAs,
    #[token("-=")]
    SubAs,
    #[token("/=")]
    DivAs,
    #[token("*=")]
    MulAs,
    #[token("%=")]
    ModAs,
    #[token("&=")]
    AndAs,

    #[regex("[a-zA-Z]+")]
    Symbol,
    #[regex("[1-9][0-9]*|0")]
    Num,
    #[regex("[1-9][0-9]*.[0-9]+|0.[0-9]+")]
    Decimal,

    #[token("\n")]
    NewLine,
    #[regex(r"[ \t\r\f]+", logos::skip)]
    #[error]
    Error,
}

impl Token {
    pub fn is_kind(self, tok: Token) -> bool {
        return tok == self;
    }
    pub fn is_of_kind(self, tokens: &[Token]) -> bool {
        return tokens.iter().any(|t| *t == self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tokenizes() {
        let mut lexer = Token::lexer("let x = 5;");
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Symbol));
        assert_eq!(lexer.next(), Some(Token::As));
        assert_eq!(lexer.next(), Some(Token::Num));
        assert_eq!(lexer.next(), Some(Token::SColon));
    }
    #[test]
    fn it_tokenizes_nums() {
        let mut lexer1 = Token::lexer("5");
        let mut lexer2 = Token::lexer("50");
        let mut lexer3 = Token::lexer("0");
        assert_eq!(lexer1.next(), Some(Token::Num));
        assert_eq!(lexer2.next(), Some(Token::Num));
        assert_eq!(lexer3.next(), Some(Token::Num));
    }
    #[test]
    fn it_tokenizes_decimals() {
        let mut lexer1 = Token::lexer("5.0");
        let mut lexer2 = Token::lexer("50.0");
        let mut lexer3 = Token::lexer("0.0");
        let mut lexer4 = Token::lexer("0.1");
        let mut lexer5 = Token::lexer(".1");
        let mut lexer6 = Token::lexer("1.");
        let mut lexer7 = Token::lexer("01.2");
        let mut lexer8 = Token::lexer("1.00");
        assert_eq!(lexer1.next(), Some(Token::Decimal));
        assert_eq!(lexer2.next(), Some(Token::Decimal));
        assert_eq!(lexer3.next(), Some(Token::Decimal));
        assert_eq!(lexer4.next(), Some(Token::Decimal));
        assert_eq!(lexer5.next(), Some(Token::Dot));
        assert_eq!(lexer5.next(), Some(Token::Num));
        assert_eq!(lexer6.next(), Some(Token::Num));
        assert_eq!(lexer6.next(), Some(Token::Dot));
        assert_eq!(lexer7.next(), Some(Token::Num));
        assert_eq!(lexer7.next(), Some(Token::Decimal));
        assert_eq!(lexer8.next(), Some(Token::Decimal));
    }
}

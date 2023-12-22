use std::fmt;

#[derive(Debug)]
pub enum Keyword {
    Class,
    Else,
    If,
    In,
    Inherits,
    Let,
    Loop,
    Pool,
    Then,
    While,
    Case,
    Esac,
    Of,
    New,
    IsVoid,
    Not,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    LessThan,
    LessThanOrEqual,
    Assign,
    Darrow,
    Dot,
    Ampersand,
    Tilda,
}

#[derive(Debug)]
pub enum Symbol {
    OpenBracket,
    CloseBracket,
    Colon,
    Semicolon,
    OpenParenthesis,
    CloseParenthesis,
    Comma,
}

#[derive(Debug)]
pub enum TokenKind<'a> {
    Keyword(Keyword),
    Operator(Operator),
    Symbol(Symbol),
    BoolConstant(bool),
    StringConstant(&'a str),
    IntegerConstant(isize),
    TypeId(&'a str),
    ObjectId(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line_number: usize,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let keyword = format!("{:?}", self).to_ascii_lowercase();
        write!(f, "{}", keyword)
    }
}

impl fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Keyword(keyword) => write!(f, "{}", keyword),
            _ => todo!(),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", self.line_number, self.kind)
    }
}

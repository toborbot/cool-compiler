use std::fmt;

#[derive(Debug)]
pub enum Keyword {
    Class,
    Else,
    Fi,
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
pub enum TokenKind<'a> {
    Keyword(Keyword),
    Operator(Operator),
    Symbol(char),
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
        let keyword = format!("{:?}", self).to_ascii_uppercase();
        write!(f, "{}", keyword)
    }
}

impl fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Keyword(keyword) => write!(f, "{}", keyword),
            TokenKind::BoolConstant(true_or_false) => write!(f, "BOOL_CONST {}", true_or_false),
            TokenKind::TypeId(id) => write!(f, "TYPEID {}", id),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", self.line_number, self.kind)
    }
}

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

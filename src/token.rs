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
    StringConstant(Vec<char>),
    UnparsedIntConstant(&'a str),
    TypeId(&'a str),
    ObjectId(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line_number: usize,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Operator::*;
        let op = match self {
            Add => "'+'",
            Subtract => "'-'",
            Multiply => "'*'",
            Divide => "'/'",
            Equal => "'='",
            LessThan => "'<'",
            LessThanOrEqual => "LE",
            Assign => "ASSIGN",
            Darrow => "DARROW",
            Dot => "'.'",
            Ampersand => "'@'",
            Tilda => "'~'",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let keyword = format!("{:?}", self).to_ascii_uppercase();
        write!(f, "{}", keyword)
    }
}

pub fn escaped_str_of_char(c: &char) -> Option<&str> {
    match c {
        '"' => Some("\\\""),
        '\n' => Some("\\n"),
        '\t' => Some("\\t"),
        '\\' => Some("\\\\"),
        '\u{0000}' => Some("\\000"),
        '\u{0001}' => Some("\\001"),
        '\u{0002}' => Some("\\002"),
        '\u{0003}' => Some("\\003"),
        '\u{0004}' => Some("\\004"),
        '\u{0008}' => Some("\\b"),
        '\u{000B}' => Some("\\013"),
        '\u{000C}' => Some("\\f"),
        '\u{000D}' => Some("\\015"),
        '\u{0012}' => Some("\\022"),
        '\u{001B}' => Some("\\033"),
        _ => None,
    }
}

impl fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind::*;
        match self {
            BoolConstant(true_or_false) => write!(f, "BOOL_CONST {true_or_false}"),
            Keyword(keyword) => write!(f, "{keyword}"),
            UnparsedIntConstant(i) => write!(f, "INT_CONST {i}"),
            ObjectId(id) => write!(f, "OBJECTID {id}"),
            Operator(op) => write!(f, "{op}"),
            TypeId(id) => write!(f, "TYPEID {id}"),
            Symbol(symbol) => write!(f, "'{symbol}'"),
            StringConstant(chars) => {
                write!(f, "STR_CONST \"")?;
                for c in chars {
                    match escaped_str_of_char(c) {
                        Some(s) => write!(f, "{s}")?,
                        None => write!(f, "{c}")?,
                    }
                }
                write!(f, "\"")
            }
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", self.line_number, self.kind)
    }
}

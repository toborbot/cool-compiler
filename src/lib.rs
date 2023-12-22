use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use token::{Keyword, Operator, Token, TokenKind};

#[derive(Parser)]
#[grammar = "cool.pest"]
pub struct CoolParser;

pub mod token;

#[derive(Debug)]
pub enum ScannerErrorKind {
    EofInComment,
    EofInStringConstant,
    InvalidCharacter(u8),
    StringContainsEscapedNullCharacter,
    StringContainsNullCharacter,
    StringConstantTooLong,
    UnclosedComment,
    UnterminatedStringConstant,
    Unknown,
}

#[derive(Debug)]
pub struct ScannerError {
    pub kind: ScannerErrorKind,
    pub line_number: usize,
}

impl ScannerError {
    fn new_unknown_error(line_number: usize) -> Self {
        ScannerError {
            kind: ScannerErrorKind::Unknown,
            line_number,
        }
    }
}

pub trait LineRange {
    fn start_line(&self) -> usize;
    fn end_line(self) -> usize;
}

impl LineRange for Pair<'_, Rule> {
    fn start_line(&self) -> usize {
        self.line_col().0
    }

    fn end_line(self) -> usize {
        for t in self.tokens() {
            match t {
                pest::Token::Start { .. } => (),
                pest::Token::End { rule: _, pos } => return pos.line_col().0,
            }
        }
        panic!("No end token found in pair.");
    }
}

fn tokenize_bool_const(bool_const: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let mut line_number = bool_const.line_col().0;
    for true_or_false in bool_const.into_inner() {
        let kind = match true_or_false.as_rule() {
            Rule::r#true => TokenKind::BoolConstant(true),
            Rule::r#false => TokenKind::BoolConstant(false),
            _ => break,
        };
        line_number = true_or_false.end_line();
        return Ok(Token { kind, line_number });
    }
    return Err(ScannerError::new_unknown_error(line_number));
}

fn tokenize_identifier(identifier: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let mut line_number = identifier.line_col().0;
    for object_or_type_id in identifier.into_inner() {
        let value = object_or_type_id.as_str();
        let kind = match object_or_type_id.as_rule() {
            Rule::objectid => TokenKind::ObjectId(value),
            Rule::typeid => TokenKind::TypeId(value),
            _ => break,
        };
        line_number = object_or_type_id.end_line();
        return Ok(Token { kind, line_number });
    }
    return Err(ScannerError::new_unknown_error(line_number));
}

fn tokenize_int_const(int_const: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let mut line_number = int_const.line_col().0;
    match int_const.as_str().parse() {
        Ok(value) => {
            let kind = TokenKind::IntegerConstant(value);
            line_number = int_const.end_line();
            Ok(Token { kind, line_number })
        }
        Err(_) => Err(ScannerError::new_unknown_error(line_number)),
    }
}

fn tokenize_keyword(keyword: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let mut line_number = keyword.line_col().0;
    for word in keyword.into_inner() {
        let kind = match word.as_rule() {
            Rule::class => TokenKind::Keyword(Keyword::Class),
            Rule::r#else => TokenKind::Keyword(Keyword::Else),
            Rule::r#if => TokenKind::Keyword(Keyword::If),
            Rule::fi => TokenKind::Keyword(Keyword::Fi),
            Rule::r#in => TokenKind::Keyword(Keyword::In),
            Rule::inherits => TokenKind::Keyword(Keyword::Inherits),
            Rule::r#let => TokenKind::Keyword(Keyword::Let),
            Rule::r#loop => TokenKind::Keyword(Keyword::Loop),
            Rule::pool => TokenKind::Keyword(Keyword::Pool),
            Rule::then => TokenKind::Keyword(Keyword::Then),
            Rule::r#while => TokenKind::Keyword(Keyword::While),
            Rule::case => TokenKind::Keyword(Keyword::Case),
            Rule::esac => TokenKind::Keyword(Keyword::Esac),
            Rule::of => TokenKind::Keyword(Keyword::Of),
            Rule::new => TokenKind::Keyword(Keyword::New),
            Rule::isvoid => TokenKind::Keyword(Keyword::IsVoid),
            Rule::not => TokenKind::Keyword(Keyword::Not),
            _ => break,
        };
        line_number = word.end_line();
        return Ok(Token { kind, line_number });
    }
    return Err(ScannerError::new_unknown_error(line_number));
}

fn tokenize_operator(operator: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let line_number = operator.line_col().0;
    let kind = match operator.as_str() {
        "+" => TokenKind::Operator(Operator::Add),
        "-" => TokenKind::Operator(Operator::Subtract),
        "*" => TokenKind::Operator(Operator::Multiply),
        "/" => TokenKind::Operator(Operator::Divide),
        "=" => TokenKind::Operator(Operator::Equal),
        "<" => TokenKind::Operator(Operator::LessThan),
        "<=" => TokenKind::Operator(Operator::LessThanOrEqual),
        "<-" => TokenKind::Operator(Operator::Assign),
        "=>" => TokenKind::Operator(Operator::Darrow),
        "." => TokenKind::Operator(Operator::Dot),
        "@" => TokenKind::Operator(Operator::Ampersand),
        "~" => TokenKind::Operator(Operator::Tilda),
        _ => {
            return Err(ScannerError::new_unknown_error(line_number));
        }
    };
    return Ok(Token { kind, line_number });
}

fn tokenize_str_const(str_const: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let mut line_number = str_const.line_col().0;
    for inner_str in str_const.into_inner() {
        match inner_str.as_rule() {
            Rule::inner_str => {
                let value = inner_str.as_str();
                line_number = inner_str.end_line();
                let kind = TokenKind::StringConstant(value);
                return Ok(Token { kind, line_number });
            }
            _ => continue,
        }
    }
    return Err(ScannerError::new_unknown_error(line_number));
}

fn tokenize_symbol(symbol: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let line_number = symbol.line_col().0;
    match symbol.as_str().chars().next() {
        Some(value) => {
            let kind = TokenKind::Symbol(value);
            Ok(Token { kind, line_number })
        }
        None => Err(ScannerError::new_unknown_error(line_number)),
    }
}

pub fn tokenize(text: &str) -> Vec<Result<Token, ScannerError>> {
    let mut tokens: Vec<_> = vec![];
    let pairs = CoolParser::parse(Rule::file, text).unwrap();
    for pair in pairs {
        for token_pair in pair.into_inner() {
            let token = match token_pair.as_rule() {
                Rule::bool_const => tokenize_bool_const(token_pair),
                Rule::int_const => tokenize_int_const(token_pair),
                Rule::identifier => tokenize_identifier(token_pair),
                Rule::keyword => tokenize_keyword(token_pair),
                Rule::operator => tokenize_operator(token_pair),
                Rule::str_const => tokenize_str_const(token_pair),
                Rule::symbol => tokenize_symbol(token_pair),
                Rule::error_eof_in_str => Err(ScannerError {
                    kind: ScannerErrorKind::EofInStringConstant,
                    line_number: token_pair.end_line(),
                }),
                Rule::error_unterminated_str => Err(ScannerError {
                    kind: ScannerErrorKind::UnterminatedStringConstant,
                    line_number: token_pair.end_line(),
                }),
                _ => {
                    dbg!(&token_pair.line_col());
                    dbg!(&token_pair);
                    todo!();
                }
            };
            tokens.push(token);
        }
    }
    tokens
}

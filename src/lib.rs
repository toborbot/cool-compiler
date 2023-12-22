use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use token::{Keyword, Token, TokenKind};

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
    UnclosedStringConstant,
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

fn end_line_number(pair: Pair<'_, Rule>) -> usize {
    for t in pair.tokens() {
        match t {
            pest::Token::Start { .. } => (),
            pest::Token::End { rule: _, pos } => return pos.line_col().0,
        }
    }
    panic!("No end token found in pair.");
}

fn tokenize_bool_const(bool_const_t: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let mut line_number = bool_const_t.line_col().0;
    for bool_const in bool_const_t.into_inner() {
        for true_or_false in bool_const.into_inner() {
            let kind = match true_or_false.as_rule() {
                Rule::r#true => TokenKind::BoolConstant(true),
                Rule::r#false => TokenKind::BoolConstant(false),
                _ => break,
            };
            line_number = end_line_number(true_or_false);
            return Ok(Token { kind, line_number });
        }
    }
    return Err(ScannerError::new_unknown_error(line_number));
}

fn tokenize_keyword(keyword: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let mut line_number = keyword.line_col().0;
    for word_t in keyword.into_inner() {
        for word in word_t.into_inner() {
            let kind = match word.as_rule() {
                Rule::r#else => TokenKind::Keyword(Keyword::Else),
                _ => break,
            };
            line_number = end_line_number(word);
            return Ok(Token { kind, line_number });
        }
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
        line_number = end_line_number(object_or_type_id);
        return Ok(Token { kind, line_number });
    }
    return Err(ScannerError::new_unknown_error(line_number));
}

pub fn tokenize(text: &str) -> Result<Vec<Token>, ScannerError> {
    let mut tokens: Vec<token::Token> = vec![];

    let pairs = CoolParser::parse(Rule::file, text).unwrap();
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            let token_or_error = match inner_pair.as_rule() {
                Rule::bool_const_t => tokenize_bool_const(inner_pair),
                Rule::identifier => tokenize_identifier(inner_pair),
                Rule::keyword => tokenize_keyword(inner_pair),
                _ => todo!(),
            };
            match token_or_error {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            }
        }
    }

    Ok(tokens)
}

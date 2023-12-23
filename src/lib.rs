use std::fmt;

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
    InvalidCharacter(char),
    StringContainsEscapedNullCharacter,
    StringContainsNullCharacter,
    StringConstantTooLong,
    UnclosedComment,
    UnterminatedStringConstant,
    Unknown,
}

impl fmt::Display for ScannerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ScannerErrorKind::*;
        if let InvalidCharacter(c) = self {
            match token::escaped_str_of_char(c) {
                Some(s) => write!(f, "ERROR \"{s}\"")?,
                None => write!(f, "ERROR \"{c}\"")?,
            }
            return Ok(());
        }
        let message = match self {
            EofInComment => "EOF in comment",
            EofInStringConstant => "EOF in string constant",
            StringContainsEscapedNullCharacter => "String contains escaped null character.",
            StringContainsNullCharacter => "String contains null character.",
            StringConstantTooLong => "String constant too long",
            UnclosedComment => "Unmatched *)",
            UnterminatedStringConstant => "Unterminated string constant",
            Unknown => "Unknown",
            InvalidCharacter(_) => panic!("Unreachable case"),
        };
        write!(f, "ERROR \"{message}\"")
    }
}

#[derive(Debug)]
pub struct ScannerError {
    pub kind: ScannerErrorKind,
    pub line_number: usize,
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", self.line_number, self.kind)
    }
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
        let mut end_line = 0;
        for t in self.tokens() {
            match t {
                pest::Token::Start { .. } => (),
                pest::Token::End { rule: _, pos } => end_line = pos.line_col().0,
            }
        }
        end_line
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
    let line_number = int_const.line_col().0;
    let kind = TokenKind::UnparsedIntConstant(int_const.as_str());
    Ok(Token { kind, line_number })
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

fn char_of_str_element(str_element: Pair<'_, Rule>) -> Option<char> {
    let escaped_character = str_element.into_inner().next()?;
    match escaped_character.as_rule() {
        Rule::r#char => escaped_character.as_str().chars().next(),
        Rule::escaped_quote => Some('"'),
        Rule::escaped_newline => Some('\n'),
        Rule::escaped_tab => Some('\t'),
        Rule::escaped_backslash => Some('\\'),
        Rule::escaped_backspace => Some('\u{0008}'),
        Rule::escaped_formfeed => Some('\u{000C}'),
        Rule::multiline => Some('\n'),
        _ => None,
    }
}

fn tokenize_str_const(str_const: Pair<'_, Rule>) -> Result<Token, ScannerError> {
    let line_number = str_const.clone().end_line();
    for inner_str in str_const.into_inner() {
        match inner_str.as_rule() {
            Rule::double_quote => continue,
            Rule::inner_str => {
                let mut characters = vec![];
                for str_element in inner_str.into_inner() {
                    match char_of_str_element(str_element) {
                        Some(c) => characters.push(c),
                        None => return Err(ScannerError::new_unknown_error(line_number)),
                    }
                }
                if characters.len() > 1024 {
                    return Err(ScannerError {
                        kind: ScannerErrorKind::StringConstantTooLong,
                        line_number,
                    });
                }
                let kind = TokenKind::StringConstant(characters);
                return Ok(Token { kind, line_number });
            }
            _ => break,
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
            use ScannerErrorKind::*;
            let token = match token_pair.as_rule() {
                Rule::bool_const => tokenize_bool_const(token_pair),
                Rule::int_const => tokenize_int_const(token_pair),
                Rule::identifier => tokenize_identifier(token_pair),
                Rule::keyword => tokenize_keyword(token_pair),
                Rule::operator => tokenize_operator(token_pair),
                Rule::str_const => tokenize_str_const(token_pair),
                Rule::symbol => tokenize_symbol(token_pair),
                Rule::error_eof_in_comment => Err(ScannerError {
                    kind: EofInComment,
                    line_number: token_pair.end_line(),
                }),
                Rule::error_unclosed_comment => Err(ScannerError {
                    kind: UnclosedComment,
                    line_number: token_pair.end_line(),
                }),
                Rule::error_eof_in_str => Err(ScannerError {
                    kind: EofInStringConstant,
                    line_number: token_pair.end_line(),
                }),
                Rule::error_unterminated_str => Err(ScannerError {
                    kind: UnterminatedStringConstant,
                    line_number: token_pair.end_line(),
                }),
                Rule::error_escaped_null_in_str => Err(ScannerError {
                    kind: StringContainsEscapedNullCharacter,
                    line_number: token_pair.end_line(),
                }),
                Rule::error_null_in_str => Err(ScannerError {
                    kind: StringContainsNullCharacter,
                    line_number: token_pair.end_line(),
                }),
                Rule::error_invalid_char => {
                    let invalid_char = token_pair.as_str().chars().next().unwrap_or_default();
                    Err(ScannerError {
                        kind: InvalidCharacter(invalid_char),
                        line_number: token_pair.end_line(),
                    })
                }
                _ => Err(ScannerError::new_unknown_error(token_pair.end_line())),
            };
            tokens.push(token);
        }
    }
    tokens
}

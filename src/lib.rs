use std::fmt;
use std::fmt::Display;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cool.pest"]
pub struct CoolParser;

#[derive(Debug)]
pub struct Token {
    type_: String,
    value: Option<String>,
    line: usize,
}

impl Token {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let rule = pair.as_rule();
        let line = if let pest::Token::End { rule: _, pos } = pair.clone().tokens().last().unwrap()
        {
            let (line, _) = pos.line_col();
            line
        } else {
            0
        };
        let value = match rule {
            Rule::bool_const => Some(pair.as_str().to_lowercase()),
            Rule::str_const => {
                let mut s = pair.clone().into_inner();
                s.next();
                let words = s
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|pair| match pair.as_rule() {
                        Rule::word => pair
                            .as_str()
                            .to_string()
                            .replace("\t", "\\t")
                            .replace("\u{0001}", "\\001")
                            .replace("\u{0002}", "\\002")
                            .replace("\u{0003}", "\\003")
                            .replace("\u{0004}", "\\004")
                            .replace("\u{000C}", "\\f")
                            .replace("\u{0008}", "\\b")
                            .replace("\u{0012}", "\\022")
                            .replace("\u{000B}", "\\013")
                            .replace("\u{001B}", "\\033")
                            .replace("\r", "\\015"),
                        Rule::escaped_tab => "\\t".to_owned(),
                        Rule::escaped_formfeed => "\\f".to_owned(),
                        Rule::escaped_newline => "\\n".to_owned(),
                        Rule::escaped_backspace => "\\b".to_owned(),
                        Rule::escaped_quote => "\\\"".to_owned(),
                        Rule::escaped_backslash => "\\\\".to_owned(),
                        Rule::multiline => "\\n".to_owned(),
                        _ => pair.to_string(),
                    })
                    .collect::<Vec<_>>();

                if words.len() > 1024 {
                    return Token {
                        type_: "ERROR".to_owned(),
                        value: Some("\"String constant too long\"".to_owned()),
                        line,
                    };
                }

                if words.contains(&("\\".to_owned() + "\0").to_string()) {
                    return Token {
                        type_: "ERROR".to_owned(),
                        value: Some("\"String contains escaped null character.\"".to_owned()),
                        line,
                    };
                }

                if words.contains(&"\0".to_string()) {
                    return Token {
                        type_: "ERROR".to_owned(),
                        value: Some("\"String contains null character.\"".to_owned()),
                        line,
                    };
                }

                Some("\"".to_owned() + &words.join("") + "\"")
            }
            Rule::int_const | Rule::objectid | Rule::typeid => Some(pair.as_str().to_string()),
            Rule::error => Some(format!("\"{}\"", pair.as_str().to_string())),
            Rule::unmatched_close_comment => Some("\"Unmatched *)\"".to_string()),
            Rule::unterminated_comment => Some("\"EOF in comment\"".to_string()),
            Rule::unterminated_str => Some("\"Unterminated string constant\"".to_string()),
            Rule::eof_in_str => Some("\"EOF in string constant\"".to_string()),
            Rule::error_backslash => Some("\"\\\\\"".to_string()),
            _ => None,
        };
        let type_ = match rule {
            Rule::special_character | Rule::operator => format!("'{}'", pair.as_str().to_string()),
            Rule::unmatched_close_comment
            | Rule::unterminated_str
            | Rule::unterminated_comment
            | Rule::eof_in_str
            | Rule::error_backslash => "ERROR".to_string(),
            _ => format!("{:?}", rule),
        };
        Token { type_, value, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rule_name = format!("{}", self.type_).to_uppercase();
        match &self.value {
            None => write!(f, "#{} {}", self.line, rule_name),
            Some(content) => write!(f, "#{} {} {}", self.line, rule_name, content),
        }
    }
}

pub fn tokenize(unparsed: &str) -> Vec<Token> {
    let pairs = CoolParser::parse(Rule::file, unparsed).unwrap();
    pairs
        .into_iter()
        .filter_map(|pair| match pair.as_rule() {
            Rule::token => {
                let token_t = pair.clone().into_inner().next().unwrap();
                let token = token_t.into_inner().next();
                match token {
                    Some(token) => Some(Token::from_pair(token)),
                    None => {
                        panic!("Error: {:?}:\n {:?}", pair.line_col(), pair);
                    }
                }
            }
            _ => None,
        })
        .collect()
}

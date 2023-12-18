use std::fmt;
use std::fmt::Display;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cool.pest"]
pub struct CoolParser;

#[derive(Debug)]
pub struct Token {
    type_: Rule,
    value: Option<String>,
    line: usize,
}

impl Token {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let rule = pair.as_rule();
        let (line, _) = pair.line_col();
        let value = match rule {
            Rule::r#else => None,
            Rule::bool_const => Some(pair.as_str().to_lowercase()),
            _ => Some(pair.as_str().to_string()),
        };
        Token {
            type_: rule,
            value,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rule_name = format!("{:?}", self.type_).to_uppercase();
        match &self.value {
            None => write!(f, "#{} {}", self.line, rule_name),
            Some(content) => write!(f, "#{} {} {}", self.line, rule_name, content),
        }
    }
}

pub fn tokenize(unparsed: &str) -> Vec<Token> {
    let pairs = CoolParser::parse(Rule::file, unparsed).unwrap();
    println!("{:?}", pairs);
    pairs
        .into_iter()
        .filter_map(|pair| {
            println!("{:?}", pair);
            if let Rule::token = pair.as_rule() {
                for token in pair.into_inner() {
                    return Some(Token::from_pair(token));
                }
            }
            None
        })
        .collect()
}

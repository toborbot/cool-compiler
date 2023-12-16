use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cool.pest"]
pub struct CoolParser;

pub struct Token {
    rule: Rule,
    value: Option<String>,
    line: usize,
}

impl Token {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let rule = pair.as_rule();
        let (line, _) = pair.line_col();
        let value = match rule {
            Rule::r#else => None,
            _ => Some(pair.as_str().to_string()),
        };
        Token { rule, value, line }
    }
}

pub fn tokenize(unparsed: &str) -> Vec<Token> {
    let pairs = CoolParser::parse(Rule::file, unparsed)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();

    pairs
        .filter_map(|pair| match pair.as_rule() {
            Rule::token => Some(Token::from_pair(pair)),

            _ => None,
        })
        .collect()
}

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cool.pest"]
pub struct CoolParser;

pub fn tokenize(unparsed: &str) -> String {
    let pairs = CoolParser::parse(Rule::file, unparsed)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();

    pairs
        .map(|pair| {
            let (line, _) = pair.line_col();
            let rule = format!("{:?}", pair.as_rule()).to_ascii_uppercase();
            match pair.as_rule() {
                Rule::r#else => format!("#{} {}", line, rule),
                Rule::bool_const => {
                    format!("#{} {} {}", line, rule, pair.as_str().to_ascii_lowercase())
                }
                Rule::EOI => "".to_string(),
                _ => format!("#{} {} {}", line, rule, pair.as_str()),
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

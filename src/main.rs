use std::fs;

fn main() {
    let unparsed = fs::read_to_string("tests/data/all_else_true.cl.cool").unwrap();
    let tokens = scanner::tokenize(&unparsed);
    println!("{tokens}");
}

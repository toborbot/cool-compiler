use std::{env::args, fs};

fn main() {
    let mut args = args();
    args.next();
    let filename = args.next().unwrap();
    let unparsed = fs::read_to_string(filename).unwrap();
    match scanner::tokenize(&unparsed) {
        Ok(tokens) => {
            println!(tokens);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

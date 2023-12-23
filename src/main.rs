use std::{env::args, fs};

fn main() {
    let mut args = args();
    args.next();
    let filename = args.next().unwrap();
    let unparsed = fs::read_to_string(filename).unwrap();
    for token_or_error in scanner::tokenize(&unparsed) {
        match token_or_error {
            Ok(token) => println!("{token}"),
            Err(e) => println!("{e}"),
        }
    }
}

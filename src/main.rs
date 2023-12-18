use std::{env::args, fs};

fn main() {
    let mut args = args();
    args.next();
    let filename = args.next().unwrap();
    let unparsed = fs::read_to_string(filename).unwrap();
    let tokens = scanner::tokenize(&unparsed);
    for token in tokens {
        println!("{}", token);
    }
}

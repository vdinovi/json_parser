mod token;
mod parser;

use std::env;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use token::tokenize;
use token::token::Token;
use parser::types::Value;
use parser::parse;

fn main() -> Result<(), Box<std::error::Error>> {
    const USAGE: &str = "usage: json_parser path/to/json/file";
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{}", USAGE);
        std::process::exit(1);
    }
    let rel_path = PathBuf::from(args[1].to_string());
    let full_path = rel_path.canonicalize()?;
    println!("Parsing file: {:?}", full_path);
    let file = File::open(full_path)?;
    let mut reader = BufReader::new(file);
    let tokens: Vec<Token> = tokenize(&mut reader)?;
    let value: Value = parse(tokens)?;
    print!("{}\n", value.to_pretty_string(0));
    Ok(())
}

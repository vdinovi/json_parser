use std::env;
use std::path::PathBuf;
use std::io::{self, BufReader};
use std::fs::File;


enum TokenType {
    L_BRACE,
    R_BRACE,
    COLON,
    STRING,
    NUMBER
}

struct Token {
    type: TokenType
};

fn tokenize(BufReader instream) -> Vec<Token> {
    // TODO
}

fn main() {
    const usage: &str = "usage: json_parser path/to/json/file";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", usage);
        std::process::exit(1);
    }

    let rel_path = PathBuf::from(args[1].to_string());
    let full_path = match rel_path.canonicalize() {
        Ok(v) => v,
        Err(e) => {
            println!("File Error: {}", e);
            std::process::exit(1);
        }
    };
    println!("Parsing file: {:?}", full_path);
    let file = match File::open(full_path) {
        Ok(v) => v,
        Err(e) => {
            println!("File Error: {}", e);
            std::process::exit(1);
        }
    };
    let mut reader = BufReader::new(file);
    

    
}

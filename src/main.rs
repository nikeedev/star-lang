use std::{
    fs,
    env,
    io,
    str
};

use regex::Regex;

#[derive(Debug)]
enum TokenType {
    Number,
    Plus,
    Minus,
    Equals
}

#[derive(Debug)]
struct Token {
    value: String,
    value_type: TokenType,
}

fn parse_string(s: &str) -> Vec<Token> {
    let tokens: Vec<Token> = Vec::new();

    // tokens;

    todo!()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("{:#?}", args);

    let file = match fs::read_to_string(args[1].clone()) {
        Ok(x) => x,
        Err(x) => panic!("Error reading file: {}", x)
    };

    // let parsed = parse_string(&file);
    // println!("{:#?}", parsed);

    let re = Regex::new(r"").unwrap();

    let s = String::from("1 + 2").char_indices(); 

    assert!(re.is_match(s.next().unwrap()));

    Ok(())
}

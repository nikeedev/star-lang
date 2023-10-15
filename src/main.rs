use std::{env, fs, io, str::{self, Chars}};

use colored::Colorize;

#[derive(Debug, Clone, Copy)]
enum TokenType {
    Number, // 0-9
    Whitespace,

    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

    Equals,

    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    token_type: TokenType,
    value: &'a str,
}

struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    chars: Chars<'a>
}

const EOF: char = '\0';

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            chars: src.chars(),
        }
    }

    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or_default()
    }

    pub fn bump(&mut self) -> char {
        match self.chars.next() {
            Some(c) => {
                self.pos += c.len_utf8();
                c
            }
            None => '\0',
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn src(&self) -> &'a str  {
        self.src
    }

    pub fn token(&mut self) -> Option<Token<'a>> {
        let start = self.pos();

        let token_type = match self.bump() {
            EOF => return None,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,

            '=' => TokenType::Equals,

            c if is_digit(c) => self.integer(),
            c if is_whitespace(c) => self.whitespace(),

            _ => TokenType::Unknown,
        };

        Some(Token {
            token_type,
            value: &self.src()[start..self.pos()],
        })
    }

    fn integer(&mut self) -> TokenType {
        while is_digit(self.peek()) {
            self.bump();
        }
        TokenType::Number
    }

    fn whitespace(&mut self) -> TokenType {
        while is_whitespace(self.peek()) {
            self.bump();
        }
        TokenType::Whitespace
    }

}

pub fn create_tokens(file: &str)-> Vec<Token<'_>> {
    Lexer::new(file).collect()
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("{:#?}", args);


    let file: String = if args.len() >= 2 {
        match fs::read_to_string(args[1].clone()) {
            Ok(x) => x,
            Err(x) => panic!("Error reading file: {}", x)
        }
    } else {
        println!("Usage: star-lang <file>.star");
        std::process::exit(0);
    };

    let tokens: Vec<Token> = create_tokens(file.as_str());

    for token in tokens {
        match token.token_type {
            TokenType::Unknown => {
                eprintln!("{}", format!("ERROR: value {} is undefined", token.value).red());
                std::process::exit(65);
            },
            _ => println!("{:#?}", token)
        }
    }

    Ok(())
}

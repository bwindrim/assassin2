use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Token {
    Empty,
    Label(String),
    Name(String),
    Instruction(String),
    Directive(String),
    Signed(i16),
    Unsigned(u16),
    String(String),
    Comma,
    Hash,
    OpenParen,
    CloseParen,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    Comment(String),
}

pub fn parse(filename: &str) -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // File <filename> must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            parse_line(&line);
        }
    }
    Ok(())
}

fn tokenize(line: &str) -> Vec<Token> {
    // This is a very simple tokenizer that splits the line into tokens based on whitespace and punctuation.
    // It does not handle string literals, comments, or other complexities of a real assembler.
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let first_char = line.chars().next();
    if let Some(c) = first_char {
        if c.is_whitespace() {
            tokens.push(Token::Empty);
        }
    }
    for c in line.chars() {
        if c.is_whitespace() {
            if !current_token.is_empty() {
                tokens.push(Token::Name(current_token));
                current_token = String::new();
            }
        } else if c.is_alphanumeric() || c == '_' {
            current_token.push(c);
        } else {
            if !current_token.is_empty() {
                tokens.push(Token::Name(current_token));
                current_token = String::new();
            }
            match c {
                ',' => tokens.push(Token::Comma),
                '#' => tokens.push(Token::Hash),
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Mul),
                '/' => tokens.push(Token::Div),
                '%' => tokens.push(Token::Mod),
                '&' => tokens.push(Token::And),
                '|' => tokens.push(Token::Or),
                '^' => tokens.push(Token::Xor),
                '!' => tokens.push(Token::Not),
                '<' => tokens.push(Token::LessThan),
                '>' => tokens.push(Token::GreaterThan),
                ';' => break, // Comment starts, ignore the rest of the line
                _ => {}
            }
        }
    }
    if !current_token.is_empty() {
        tokens.push(Token::Name(current_token));
    }
    tokens
}

fn parse_line(line: &str) {
    let tokens = tokenize(line);
    println!("{:?}", tokens);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(e)
        }
    }
}

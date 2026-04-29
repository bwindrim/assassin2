use core::panic;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Token {
    Empty,
    Name(String),
    Unsigned(u16),
    String(String),
    Colon,
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
    let mut tokens = Vec::new();

    // We iterate through the characters of the line, building up tokens as we go.
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => {
                // If the line starts with whitespace then we push an Empty token to represent that.
                // (Note the two different meanings of "empty" here.)
                // Otherwise we ignore whitespace.
                if tokens.is_empty() {
                    tokens.push(Token::Empty);
                }
            }

            ';' => {
                // The rest of the line is a comment, so we push a Comment token and break.
                let mut comment_string = String::new();
                while let Some(next_c) = chars.peek() {
                    comment_string.push(*next_c);
                    chars.next(); // consume the character
                }
                tokens.push(Token::Comment(comment_string));
                break;
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut name_string = String::new();
                name_string.push(c);
                while let Some(next_c) = chars.peek()
                    && (next_c.is_alphanumeric() || *next_c == '_')
                {
                    name_string.push(*next_c);
                    chars.next(); // consume the character
                }
                tokens.push(Token::Name(name_string));
            }

            '0'..='9' => {
                let mut decimal_string = String::new();
                decimal_string.push(c);
                while let Some(next_c) = chars.peek()
                    && next_c.is_ascii_digit()
                {
                    decimal_string.push(*next_c);
                    chars.next(); // consume the character
                }
                tokens.push(Token::Unsigned(
                    u16::from_str_radix(&decimal_string, 10).unwrap(),
                ));
            }

            '$' => {
                // hexadecimal literal
                let mut hex_string = String::new();
                while let Some(next_c) = chars.peek()
                    && next_c.is_ascii_hexdigit()
                {
                    hex_string.push(*next_c);
                    chars.next(); // consume the character
                }
                tokens.push(Token::Unsigned(
                    u16::from_str_radix(&hex_string, 16).unwrap(),
                ));
            }

            '@' => {
                // binary literal
                let mut bin_string = String::new();
                while let Some(next_c) = chars.peek()
                    && (*next_c == '0' || *next_c == '1')
                {
                    bin_string.push(*next_c);
                    chars.next(); // consume the character
                }
                tokens.push(Token::Unsigned(
                    u16::from_str_radix(&bin_string, 2).unwrap(),
                ));
            }

            '"' => {
                // string literal
                let mut string_literal = String::new();
                while let Some(next_c) = chars.peek() {
                    if *next_c == '"' {
                        chars.next(); // consume the closing quote
                        break;
                    }
                    string_literal.push(*next_c);
                    chars.next(); // consume the character
                }
                tokens.push(Token::String(string_literal));
            }

            '\'' => {
                // character literal
                if let Some(next_c) = chars.next() {
                    tokens.push(Token::Unsigned(next_c as u16));
                    // Consume the closing quote
                    if let Some(quote_c) = chars.next() {
                        if quote_c != '\'' {
                            panic!("Expected closing quote for character literal");
                        }
                    } else {
                        panic!("Expected closing quote for character literal");
                    }
                } else {
                    panic!("Expected character literal after opening quote");
                }
            }

            ',' => tokens.push(Token::Comma),
            ':' => tokens.push(Token::Colon),
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
            _ => {
                panic!("Unexpected character: {}", c);
            }
        }
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

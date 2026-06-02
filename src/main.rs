pub mod parser;
pub mod generator;
pub mod representation;
pub mod recogniser;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use parser::tokenize;

use crate::recogniser::Recogniser;

fn main() {
    println!("Assassin 6809 assembler, V2.0");
    let mut recogniser = Recogniser::new();
    let _ = parse("tst/boot.a", &mut recogniser);
    println!("Namelist: {:?}", recogniser);
    println!("Done");
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

fn parse(filename: &str, recogniser: &mut Recogniser) -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // File <filename> must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            {
                let line: &str = &line;
                let result = tokenize(line);
                match result {
                    Ok(tokens) => {
                        println!("{:?}", tokens);
                        let line = recogniser.recognise_line(&tokens);
                        match line {
                            Ok(line) => println!("Recognised line: {:?}", line) ,
                            Err(e) => println!("Recognition error: {}", e),
                        }
                    },
                    Err(e) => println!("Tokenizer error: {:?}", e),
                }
            };
        }
    }
    Ok(())
}

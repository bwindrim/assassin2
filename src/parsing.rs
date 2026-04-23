use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

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

fn parse_line(line: &str) {
    line.split_whitespace().for_each(|token| {
        println!("Token: {}", token);
    });
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

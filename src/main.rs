pub mod parser;
pub mod generator;
pub mod representation;
pub mod recogniser;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use parser::tokenize;
use generator::*;
use representation::*;

fn main() {
    let mut seg = Segment {
        name: "CODE".to_string(),
        elements: vec![
            Element::Inst(Instruction::ABX),
            Element::Inst(Instruction::ADDA(Type1::IMM(42))),
            Element::Data(Data::DB(vec![1, 2, 3])),
            Element::Data(Data::DW(vec![0x1234, 0x5678])),
            Element::Data(Data::DS(16)),
        ],
    };
    let instr = Instruction::ADDA(Type1::IMM(42));
    println!("{:?} -> {:?}", instr, encode_instruction(&instr));
    seg.elements.push(Element::Inst(instr));
    let instr = Instruction::ADDD(Type1::IMM(1042));
    println!("{:?} -> {:?}", instr, encode_instruction(&instr));
    seg.elements.push(Element::Inst(instr));
    let instr = Instruction::EXG(Typext::from_tfr_exg_registers8(
        TfrExgRegister8::A,
        TfrExgRegister8::B,
    ));
    println!("{:?} -> {:?}", instr, encode_instruction(&instr));
    seg.elements.push(Element::Inst(instr));
    println!("{:?}", seg);

    let _ = parse("tst/boot.a");
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

fn parse(filename: &str) -> std::io::Result<()> {
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
                        let recognition_result = recogniser::recognise(&tokens);
                        match recognition_result {
                            Ok(Some(element)) => println!("Recognised element: {:?}", element),
                            Ok(None) => println!("No element recognised"),
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

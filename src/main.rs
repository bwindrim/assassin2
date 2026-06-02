pub mod generator;
pub mod parser;
pub mod recogniser;
pub mod representation;

use crate::recogniser::Recogniser;

fn main() {
    println!("Assassin 6809 assembler, V2.0");
    let mut recogniser = Recogniser::new();
    let _ = recogniser.parse("tst/boot.a");
    println!("Namelist: {:?}", recogniser);
    println!("Done");
}

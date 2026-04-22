pub mod representation;
pub mod generation;

use representation::*;
use generation::*;

fn main() {
    let mut seg = Segment {
        name: "CODE".to_string(),
        elements: vec![
            Element::Inst(Instruction::ABX),
            Element::Inst(Instruction::ADDA(Type1::IMM(42))),
            Element::Data(Data::DB(vec![1, 2, 3])),
            Element::Data(Data::DW(vec![0x1234, 0x5678])),
            Element::Data(Data::DS(16))
        ]
    };
    let instr = Instruction::ADDA(Type1::IMM(42));
    println!("{:?} -> {:?}", instr, encode_instruction(&instr));
    seg.elements.push(Element::Inst(instr));
    let instr = Instruction::ADDD(Type1::IMM(1042));
    println!("{:?} -> {:?}", instr, encode_instruction(&instr));
    seg.elements.push(Element::Inst(instr));
    let instr = Instruction::EXG(Typext::from_tfr_exg_registers8(TfrExgRegister8::A, TfrExgRegister8::B));
    println!("{:?} -> {:?}", instr, encode_instruction(&instr));
    seg.elements.push(Element::Inst(instr));
    println!("{:?}", seg);

}

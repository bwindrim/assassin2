use std::{collections::HashSet, vec};

// Source - https://stackoverflow.com/a/72736737
// Posted by gagiuntoli
// Retrieved 2026-04-20, License - CC BY-SA 4.0

trait IntoBytes: Sized {
    fn to_be_bytes(a: Self) -> Vec<u8>;
}

impl IntoBytes for u8 {
    fn to_be_bytes(a: Self) -> Vec<u8> {
        a.to_be_bytes().to_vec()
    }
}

impl IntoBytes for u16 {
    fn to_be_bytes(a: Self) -> Vec<u8> {
        a.to_be_bytes().to_vec()
    }
}

// The following enums and structs represent the various instruction formats and operand types for the 6809.
// The orgaisation of these types follows the principle of "Make invalid states unrepresentable", so that,
// for example, an instruction that can only take an immediate operand cannot be constructed with a direct or extended operand.
// Any instruction that can be constructed using these types is guaranteed to be valid, and any attempt to
// construct an invalid instruction will result in a compile-time error. This makes the assembler more robust
// and easier to maintain, as it eliminates the possibility of invalid instructions being generated due to programmer error
// and removes any need for error-checking in the code generation functions.
// Also, the types used to represent instructions are independent of the encoding of those instructions, so that
// the same instruction can be encoded in different ways without changing the underlying representation of the instruction itself.
// For example, the number of bytes used for the opcodes, and the bit values used to encode those opcodes, form no part of
// the instruction representation, and are handled separately in the encode_instruction function.
// The encoding of the instruction is handled separately in the encode_instruction function, which takes care of generating the correct opcode and operand bytes based on the instruction format and addressing mode.
// This separation of concerns makes the code more modular and easier to understand.

#[derive(Debug)]
enum PushPullRegister {
    A,
    B,
    X,
    Y,
    US,
    PC,
    CC,
    DP
}

#[derive(Debug)]
enum TfrExgRegister8 {
    A,
    B,
    CC,
    DP
}

#[derive(Debug)]
enum TfrExgRegister16 {
    D,
    X,
    Y,
    U,
    S,
    PC
}

#[derive(Debug)]
enum IndexRegister {
    X,
    Y,
    U,
    S
}

#[derive(Debug)]
enum AccOffsetRegister {
    A,
    B
}

#[derive(Debug)]
enum IndexedIndirect {
    CONST     { offset: u16, reg: IndexRegister },
    CONST_IND { offset: u16, reg: IndexRegister },
    ACC       { offset: AccOffsetRegister, reg: IndexRegister },
    ACC_IND   { offset: AccOffsetRegister, reg: IndexRegister },
    INC_1     { reg: IndexRegister },
    INC_2     { reg: IndexRegister },
    INC_2_IND { reg: IndexRegister },
    DEC_1     { reg: IndexRegister },
    DEC_2     { reg: IndexRegister },
    DEC_2_IND { reg: IndexRegister },
    PCR       { target: u16 },
    PC        { offset: u16 }, 
    EXT_IND(u16)
}

// Type1 instructions have a single operand,
// which can be an immediate value or a direct, extended,
// or indexed/indirect memory location.
// The type parameter T is used to distinguish between 8-bit and 16-bit immediate values.
#[derive(Debug)]
enum Type1<T: IntoBytes> {
    IMM(T),
    DIR(u8),
    EXT(u16),
    IND(IndexedIndirect),
}

// Type2 instructions have a single operand,
// which can be a direct, extended, or indexed/indirect memory location.
#[derive(Debug)]
enum Type2 {
    DIR(u8),
    EXT(u16),
    IND(IndexedIndirect),
}

impl From<IndexedIndirect> for Type2 {
    fn from(v: IndexedIndirect) -> Self {
        Self::IND(v)
    }
}

// Typebr instructions have a single operand,
// which is a branch target label.
#[derive(Debug)]
enum Typebr {
    RESOLVED(u16),
    UNRESOLVED(String)
}

// Typepspl instructions have a list (actually a set) of operands,
// which are registers to be pushed or pulled.
#[derive(Debug)]
struct Typepspl {
    registers: HashSet<PushPullRegister>
}

impl std::ops::Deref for Typepspl {
    type Target = HashSet<PushPullRegister>;

    fn deref(&self) -> &Self::Target {
        &self.registers
    }
}

// Typecc instructions have a single operand,
// which is a condition code mask.
#[derive(Debug)]
struct Typecc {
    mask: u8
}

// Typext instructions have a pair of operands,
// both of which are registers. These may be 8-bit or 16-bit registers,
// depending on the instruction, but must be the same size.
#[derive(Debug)]
enum Typext {
    BYTE( TfrExgRegister8, TfrExgRegister8 ),
    WORD( TfrExgRegister16, TfrExgRegister16 )
}

impl Typext {
    fn from_tfr_exg_registers8(r1: TfrExgRegister8, r2: TfrExgRegister8) -> Self {
        Self::BYTE(r1, r2)
    }

    fn from_tfr_exg_registers16(r1: TfrExgRegister16, r2: TfrExgRegister16) -> Self {
        Self::WORD(r1, r2)
    }
}

#[derive(Debug)]
enum Instruction {
    ABX,
    ADCA(Type1<u8>),
    ADCB(Type1<u8>),
    ADDA(Type1<u8>),
    ADDB(Type1<u8>),
    ADDD(Type1<u16>),
    ANDA(Type1<u8>),
    ANDB(Type1<u8>),
    ANDCC(Typecc),
    ASLA,
    ASLB,
    ASL(Type2),
    ASRA,
    ASRB,
    ASR(Type2),
    BCC(Typebr),
    BCS(Typebr),
    BEQ(Typebr),
    BGE(Typebr),
    BGT(Typebr),
    BHI(Typebr),
    BHS(Typebr),
    BITA(Type1<u8>),
    BITB(Type1<u8>),
    BLE(Typebr),
    BLO(Typebr),
    BLS(Typebr),
    BLT(Typebr),
    BMI(Typebr),
    BNE(Typebr),
    BPL(Typebr),
    BRA(Typebr),
    BRN(Typebr),
    BSR(Typebr),
    BVC(Typebr),
    BVS(Typebr),
    CLC,
    CLF,
    CLI,
    CLIF,
    CLRA,
    CLRB,
    CLR(Type2),
    CLV,
    CMPA(Type1<u8>),
    CMPB(Type1<u8>),
    CMPD(Type1<u16>),
    CMPS(Type1<u16>),
    CMPU(Type1<u16>),
    CMPX(Type1<u16>),
    CMPY(Type1<u16>),
    COMA,
    COMB,
    COM(Type2),
    CWAI(Typecc),
    DAA,
    DECA,
    DECB,
    DEC(Type2),
    EORA(Type1<u8>),
    EORB(Type1<u8>),
    EXG(Typext),
    INCA,
    INCB,
    INC(Type2),
    JMP(Type2),
    JSR(Type1<u8>),
    LBCC(Typebr),
    LBCS(Typebr),
    LBEQ(Typebr),
    LBGE(Typebr),
    LBGT(Typebr),
    LBHI(Typebr),
    LBHS(Typebr),
    LBLE(Typebr),
    LBLO(Typebr),
    LBLS(Typebr),
    LBLT(Typebr),
    LBMI(Typebr),
    LBNE(Typebr),
    LBPL(Typebr),
    LBRA(Typebr),
    LBRN(Typebr),
    LBSR(Typebr),
    LBVC(Typebr),
    LBVS(Typebr),
    LDA(Type1<u8>),
    LDB(Type1<u8>),
    LDD(Type1<u16>),
    LDS(Type1<u16>),
    LDU(Type1<u16>),
    LDX(Type1<u16>),
    LDY(Type1<u16>),
    LEAS(Type2),
    LEAU(Type2),
    LEAX(Type2),
    LEAY(Type2),
    LSLA,
    LSLB,
    LSL(Type2),
    LSRA,
    LSRB,
    LSR(Type2),
    MUL,
    NEGA,
    NEGB,
    NEG(Type2),
    NOP,
    ORA(Type1<u8>),
    ORB(Type1<u8>),
    ORCC(Typecc),
    PSHU(Typepspl),
    PSHS(Typepspl),
    PULU(Typepspl),
    PULS(Typepspl),
    ROLA,
    ROLB,
    ROL(Type2),
    RORA,
    RORB,
    ROR(Type2),
    RTI,
    RTS,
    SBCA(Type1<u8>),
    SBCB(Type1<u8>),
    SBCC(Typebr),
    SBCS(Typebr),
    SBEQ(Typebr),
    SBLT(Typebr),
    SBLE(Typebr),
    SBGT(Typebr),
    SBGE(Typebr),
    SBLO(Typebr),
    SBLS(Typebr),
    SBMI(Typebr),
    SBNE(Typebr),
    SBPL(Typebr),
    SBRA(Typebr),
    SBRN(Typebr),
    SBSR(Typebr),
    SBVC(Typebr),
    SBVS(Typebr),
    SEC,
    SEF,
    SEI,
    SEIF,
    SEV,
    SEX,
    STA(Type2),
    STB(Type2),
    STD(Type2),
    STS(Type2),
    STU(Type2),
    STX(Type2),
    STY(Type2),
    SUBA(Type1<u8>),
    SUBB(Type1<u8>),
    SUBD(Type1<u16>),
    SWI,
    SWI2,
    SWI3,
    SYNC,
    TFR(Typext),
    TSTA,
    TSTB,
    TST(Type2)
}

#[derive(Debug)]
enum Data {
    DB(Vec<u8>),
    DW(Vec<u16>),
    DS(usize)
}
#[derive(Debug)]
enum Element {
    Inst(Instruction),
    Data(Data)
}
#[derive(Debug)]
struct Segment {
    name: String,
    elements: Vec<Element>
}

fn gen_bytes<T: IntoBytes + Copy>(a: T) -> Vec<u8> {
    T::to_be_bytes(a)
}

fn encode_type0(opcode: u16) -> Vec<u8> {
    if opcode > 0xFF {
        vec![(opcode >> 8) as u8, opcode as u8]
    } else {
        vec![opcode as u8]
    }
}

fn encode_type1_opcode<T: IntoBytes + Copy>(opcode: u16, operand: &Type1<T>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    if opcode > 0xFF {
        bytes.push((opcode >> 8) as u8);
    }
    bytes.extend(match operand {
        Type1::IMM(_) => vec![(opcode as u8) | 0x00],
        Type1::DIR(_) => vec![(opcode as u8) | 0x10],
        Type1::EXT(_) => vec![(opcode as u8) | 0x30],
        Type1::IND(_) => vec![(opcode as u8) | 0x20],
    });
    bytes
}

fn encode_type1_operand<T: IntoBytes + Copy>(operand: &Type1<T>) -> Vec<u8> {
    match operand {
        Type1::IMM(value) => gen_bytes::<T>(*value),
        Type1::DIR(addr) => vec![*addr],
        Type1::EXT(addr) => vec![(*addr >> 8) as u8, *addr as u8],
        Type1::IND(indirect) => unimplemented!("*** Indexed indirect operands are not implemented in this example ***"),
    }
}

fn encode_type1<T: IntoBytes + Copy>(opcode: u16, operand: &Type1<T>) -> Vec<u8> {
    let mut bytes = encode_type1_opcode(opcode, operand);
    bytes.extend(encode_type1_operand(operand));
    bytes
}

fn encode_instruction(instr: &Instruction) -> Vec<u8> {
    // This is a placeholder implementation. In a real assembler, this function would
    // need to handle all the different instruction formats and addressing modes.
    match instr {
        Instruction::ABX => encode_type0(0x3A), // Opcode for ABX
        Instruction::ADCA(operand)  => encode_type1(0x89, operand), // Base opcode for ADCA
        Instruction::ADCB(operand)  => encode_type1(0xC9, operand), // Base opcode for ADCB
        Instruction::ADDA(operand)  => encode_type1(0x8B, operand), // Base opcode for ADDA
        Instruction::ADDB(operand)  => encode_type1(0xCB, operand), // Base opcode for ADDB
        Instruction::ADDD(operand) => encode_type1(0xC3, operand), // Base opcode for ADDD
        Instruction::ANDA(operand)  => encode_type1(0x84, operand), // Base opcode for ANDA
        Instruction::ANDB(operand)  => encode_type1(0xC4, operand), // Base opcode for ANDB
        Instruction::ASLA => encode_type0(0x48), // Opcode for ASLA (same as LSLA)
        Instruction::ASLB => encode_type0(0x58), // Opcode for ASLB (same as LSLB)
        Instruction::ASRA => encode_type0(0x47), // Opcode for ASRA
        Instruction::ASRB => encode_type0(0x57), // Opcode for ASRB
        Instruction::BITA(operand) => encode_type1(0x85, operand), // Base opcode for BITA
        Instruction::BITB(operand) => encode_type1(0xC5, operand), // Base opcode for BITB
        Instruction::CLC  => encode_type0(0x1CFE), // ANDCC #$FE (clear carry)
        Instruction::CLF  => encode_type0(0x1CBF), // ANDCC #$BF (clear fast interrupt disable)
        Instruction::CLI  => encode_type0(0x1CEF), // ANDCC #$EF (clear interrupt disable)
        Instruction::CLIF => encode_type0(0x1CAF), // ANDCC #$AF (clear interrupt disables)
        Instruction::CLRA => encode_type0(0x4F), // Opcode for CLRA
        Instruction::CLRB => encode_type0(0x5F), // Opcode for CLRB
        Instruction::CLV  => encode_type0(0x1CFD), // ANDCC #$FD (clear overflow)
        Instruction::CMPA(operand)  => encode_type1(0x81,   operand), // Base opcode for CMPA
        Instruction::CMPB(operand)  => encode_type1(0xC1,   operand), // Base opcode for CMPB
        Instruction::CMPD(operand) => encode_type1(0x1083, operand), // Base opcode for CMPD
        Instruction::CMPS(operand) => encode_type1(0x118C, operand), // Base opcode for CMPS
        Instruction::CMPU(operand) => encode_type1(0x1183, operand), // Base opcode for CMPU
        Instruction::CMPX(operand) => encode_type1(0x8C,   operand), // Base opcode for CMPX
        Instruction::CMPY(operand) => encode_type1(0x108C, operand), // Base opcode for CMPY
        Instruction::COMA => encode_type0(0x43), // Opcode for COMA
        Instruction::COMB => encode_type0(0x53), // Opcode for COMB
        Instruction::DAA  => encode_type0(0x19), // Opcode for DAA
        Instruction::DECA => encode_type0(0x4A), // Opcode for DECA
        Instruction::DECB => encode_type0(0x5A), // Opcode for DECB
        Instruction::EORA(operand) => encode_type1(0x88, operand), // Base opcode for EORA
        Instruction::EORB(operand) => encode_type1(0xC8, operand), // Base opcode for EORB
        Instruction::INCA => encode_type0(0x4C), // Opcode for INCA
        Instruction::INCB => encode_type0(0x5C), // Opcode for INCB
        Instruction::LDA(operand)  => encode_type1(0x86,   operand), // Base opcode for LDA
        Instruction::LDB(operand)  => encode_type1(0xC6,   operand), // Base opcode for LDB
        Instruction::LDD(operand) => encode_type1(0xCC,   operand), // Base opcode for LDD
        Instruction::LDS(operand) => encode_type1(0x10CE, operand), // Base opcode for LDS
        Instruction::LDU(operand) => encode_type1(0xCE,   operand), // Base opcode for LDU
        Instruction::LDX(operand) => encode_type1(0x8E,   operand), // Base opcode for LDX
        Instruction::LDY(operand) => encode_type1(0x108E, operand), // Base opcode for LDY
        Instruction::LSLA => encode_type0(0x48), // Opcode for LSLA (same as ASLA)
        Instruction::LSLB => encode_type0(0x58), // Opcode for LSLB (same as ASLB)
        Instruction::LSRA => encode_type0(0x44), // Opcode for LSRA
        Instruction::LSRB => encode_type0(0x54), // Opcode for LSRB
        Instruction::MUL  => encode_type0(0x3D), // Opcode for MUL
        Instruction::NEGA => encode_type0(0x40), // Opcode for NEGA
        Instruction::NEGB => encode_type0(0x50), // Opcode for NEGB
        Instruction::NOP  => encode_type0(0x12), // Opcode for NOP
        Instruction::ORA(operand) => encode_type1(0x8A, operand), // Base opcode for ORA
        Instruction::ORB(operand) => encode_type1(0xCA, operand), // Base opcode for ORB
        Instruction::ROLA => encode_type0(0x49), // Opcode for ROLA
        Instruction::ROLB => encode_type0(0x59), // Opcode for ROLB
        Instruction::RORA => encode_type0(0x46), // Opcode for RORA
        Instruction::RORB => encode_type0(0x56), // Opcode for RORB
        Instruction::RTI  => encode_type0(0x3B), // Opcode for RTI
        Instruction::RTS  => encode_type0(0x39), // Opcode for RTS
        Instruction::SBCA(operand) => encode_type1(0x82, operand), // Base opcode for SBCA
        Instruction::SBCB(operand) => encode_type1(0xC2, operand), // Base opcode for SBCB
        Instruction::SEC  => encode_type0(0x1A01), // ORCC #$01 (set carry)
        Instruction::SEF  => encode_type0(0x1A40), // ORCC #$40 (set fast interrupt disable)
        Instruction::SEI  => encode_type0(0x1A10), // ORCC #$10 (set interrupt disable)
        Instruction::SEIF => encode_type0(0x1A50), // ORCC #$50 (set both interrupt disables)
        Instruction::SEV  => encode_type0(0x1A02), // ORCC #$02 (set overflow)
//        Instruction::STA(operand) => encode_type1(0x97, operand), // Base opcode for STA
//        Instruction::STB(operand) => encode_type1(0xD7, operand), // Base opcode for STB
//        Instruction::STD(operand) => encode_type1(0xDD, operand), // Base opcode for STD
//        Instruction::STS(operand) => encode_type1(0x10DF, operand), // Base opcode for STS
//        Instruction::STU(operand) => encode_type1(0xDF, operand), // Base opcode for STU
//        Instruction::STX(operand) => encode_type1(0x9F, operand), // Base opcode for STX
//        Instruction::STY(operand) => encode_type1(0x109F, operand), // Base opcode for STY
        Instruction::SUBA(operand)  => encode_type1(0x80, operand), // Base opcode for SUBA
        Instruction::SUBB(operand)  => encode_type1(0xC0, operand), // Base opcode for SUBB
        Instruction::SUBD(operand) => encode_type1(0x83, operand), // Base opcode for SUBD
        Instruction::SEX  => encode_type0(0x1D), // Opcode for SEX
        Instruction::SWI  => encode_type0(0x3F), // Opcode for SWI
        Instruction::SWI2 => encode_type0(0x103F), // Opcode for SWI2 (two-byte opcode)
        Instruction::SWI3 => encode_type0(0x113F), // Opcode for SWI3 (two-byte opcode)
        Instruction::SYNC => encode_type0(0x13), // Opcode for SYNC
        Instruction::TSTA => encode_type0(0x4D), // Opcode for TSTA
        Instruction::TSTB => encode_type0(0x5D), // Opcode for TSTB

        _ => unimplemented!("*** Instruction not implemented in this example ***"),
    }
}
fn main() {
    println!("Hello, world!");
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
    println!("{:?}", instr);
    println!("{:?}", encode_instruction(&instr));
    seg.elements.push(Element::Inst(instr));
    let instr = Instruction::ADDD(Type1::IMM(1042));
    println!("{:?}", instr);
    println!("{:?}", encode_instruction(&instr));
    seg.elements.push(Element::Inst(instr));
    let instr = Instruction::EXG(Typext::from_tfr_exg_registers8(TfrExgRegister8::A, TfrExgRegister8::B));
    println!("{:?}", instr);
    seg.elements.push(Element::Inst(instr));
    println!("{:?}", seg);

}

use std::collections::HashSet;

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
// or indexed memory location.
// The type parameter T is used to distinguish between 8-bit and 16-bit immediate values.
#[derive(Debug)]
enum Type1<T> {
    IMM(T),
    DIR(u8),
    EXT(u16),
    IND(IndexedIndirect),
}

// Type2 instructions have a single operand,
// which can be a direct, extended, or indexed memory location.
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
    seg.elements.push(Element::Inst(instr));
    let instr = Instruction::EXG(Typext::from_tfr_exg_registers8(TfrExgRegister8::A, TfrExgRegister8::B));
    println!("{:?}", instr);
    seg.elements.push(Element::Inst(instr));
    println!("{:?}", seg);

}

use std::collections::HashSet;

// Source - https://stackoverflow.com/a/72736737
// Posted by gagiuntoli
// Retrieved 2026-04-20, License - CC BY-SA 4.0

pub trait IntoBytes: Sized {
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
pub enum PushPullRegister {
    A,
    B,
    X,
    Y,
    US,
    PC,
    CC,
    DP,
}

#[derive(Debug)]
pub enum TfrExgRegister8 {
    A,
    B,
    CC,
    DP,
}

#[derive(Debug)]
pub enum TfrExgRegister16 {
    D,
    X,
    Y,
    U,
    S,
    PC,
}

#[derive(Debug)]
pub enum IndexRegister {
    X,
    Y,
    U,
    S,
}

#[derive(Debug)]
pub enum AccOffsetRegister {
    A,
    B,
    D,
}

#[derive(Debug)]
pub enum IndexedIndirect {
    Const {
        offset: i16,
        reg: IndexRegister,
    },
    ConstInd {
        offset: i16,
        reg: IndexRegister,
    },
    Acc {
        offset: AccOffsetRegister,
        reg: IndexRegister,
    },
    AccInd {
        offset: AccOffsetRegister,
        reg: IndexRegister,
    },
    Inc1 {
        reg: IndexRegister,
    },
    Inc2 {
        reg: IndexRegister,
    },
    Inc2Ind {
        reg: IndexRegister,
    },
    Dec1 {
        reg: IndexRegister,
    },
    Dec2 {
        reg: IndexRegister,
    },
    Dec2Ind {
        reg: IndexRegister,
    },
    Pcr {
        target: u16,
    },
    PcrInd {
        target: u16,
    },
    Pc {
        offset: u16,
    },
    PcInd {
        offset: u16,
    },
    ExtInd(u16),
}

// Type1 instructions have a single operand,
// which can be an immediate value or a direct, extended,
// or indexed/indirect memory location.
// The type parameter T is used to distinguish between 8-bit and 16-bit immediate values.
#[derive(Debug)]
pub enum Type1<T: IntoBytes> {
    IMM(T),
    DIR(u8),
    EXT(u16),
    IND(IndexedIndirect),
}

// Type2 instructions have a single operand,
// which can be a direct, extended, or indexed/indirect memory location.
#[derive(Debug)]
pub enum Type2 {
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
pub enum Typebr {
    SHORT(i8),
    LONG(i16),
    UNRESOLVED(String),
}

// Typepspl instructions have a list (actually a set) of operands,
// which are registers to be pushed or pulled.
#[derive(Debug)]
pub struct Typepspl {
    pub registers: HashSet<PushPullRegister>,
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
pub struct Typecc {
    pub mask: u8,
}

// Typext instructions have a pair of operands,
// both of which are registers. These may be 8-bit or 16-bit registers,
// depending on the instruction, but must be the same size.
#[derive(Debug)]
pub enum Typext {
    BYTE(TfrExgRegister8, TfrExgRegister8),
    WORD(TfrExgRegister16, TfrExgRegister16),
}

impl Typext {
    pub fn from_tfr_exg_registers8(r1: TfrExgRegister8, r2: TfrExgRegister8) -> Self {
        Self::BYTE(r1, r2)
    }

    pub fn from_tfr_exg_registers16(r1: TfrExgRegister16, r2: TfrExgRegister16) -> Self {
        Self::WORD(r1, r2)
    }
}

#[derive(Debug)]
pub enum Instruction {
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
    JSR(Type2),
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
    SBHI(Typebr),
    SBHS(Typebr),
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
    TST(Type2),
}

#[derive(Debug)]
pub enum Data {
    DB(Vec<u8>),
    DW(Vec<u16>),
    DS(usize),
}
#[derive(Debug)]
pub enum Element {
    Inst(Instruction),
    Data(Data),
}
#[derive(Debug)]
pub struct Segment {
    pub name: String,
    pub elements: Vec<Element>,
}

pub fn gen_bytes<T: IntoBytes + Copy>(a: T) -> Vec<u8> {
    T::to_be_bytes(a)
}

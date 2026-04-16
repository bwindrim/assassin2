enum Register {
    A,
    B,
    D,
    X,
    Y,
    U,
    S,
    PC,
    DP
}

struct IndexedIndirect {

}

// Type0 instructions have no operands.
enum Type0 {
}

// Type1<u8> instructions have a single operand,
// which can be an immediate value, or a direct, extended,
// or indexed memory location.
enum Type1<T> {
    IMM(T),
    DIR(u8),
    EXT(u16),
    IND(IndexedIndirect),
}

// Type2 instructions have a single operand,
// which can be a direct, extended, or indexed memory location.
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
enum Typebr {
    RESOLVED(u16),
    UNRESOLVED(String)
}

// Typepspl have a list of operands, which are registers to be pushed or pulled.
enum Typepspl {

}

// Typecc instructions have a single operand,
// which is a condition code mask.
struct Typecc {
    mask: u8
}

// Typext instructions have a pair of operands,
// both of which are registers.
struct Typext {
    src: Register,
    dst: Register
}

enum Instruction {
    ABX(Type0),
    ADCA(Type1<u8>),
    ADCB(Type1<u8>),
    ADDA(Type1<u8>),
    ADDB(Type1<u8>),
    ADDD(Type1<u16>),
    ANDA(Type1<u8>),
    ANDB(Type1<u8>),
    ANDCC(Typecc),
    ASLA(Type0),
    ASLB(Type0),
    ASL(Type2),
    ASRA(Type0),
    ASRB(Type0),
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
    CLC(Type0),
    CLF(Type0),
    CLI(Type0),
    CLIF(Type0),
    CLRA(Type0),
    CLRB(Type0),
    CLR(Type2),
    CLV(Type0),
    CMPA(Type1<u8>),
    CMPB(Type1<u8>),
    CMPD(Type1<u16>),
    CMPS(Type1<u16>),
    CMPU(Type1<u16>),
    CMPX(Type1<u16>),
    CMPY(Type1<u16>),
    COMA(Type0),
    COMB(Type0),
    COM(Type2),
    CWAI(Typecc),
    DAA(Type0),
    DECA(Type0),
    DECB(Type0),
    DEC(Type2),
    EORA(Type1<u8>),
    EORB(Type1<u8>),
    EXG(Typext),
    INCA(Type0),
    INCB(Type0),
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
    LSLA(Type0),
    LSLB(Type0),
    LSL(Type2),
    LSRA(Type0),
    LSRB(Type0),
    LSR(Type2),
    MUL(Type0),
    NEGA(Type0),
    NEGB(Type0),
    NEG(Type2),
    NOP(Type0),
    ORA(Type1<u8>),
    ORB(Type1<u8>),
    ORCC(Typecc),
    PSHU(Typepspl),
    PSHS(Typepspl),
    PULU(Typepspl),
    PULS(Typepspl),
    ROLA(Type0),
    ROLB(Type0),
    ROL(Type2),
    RORA(Type0),
    RORB(Type0),
    ROR(Type2),
    RTI(Type0),
    RTS(Type0),
    SBCA(Type1<u8>),
    SBCB(Type1<u8>),
    SBCC(Typebr),
    SBCS(Typebr),
    SBEQ(Typebr),
    SBLT(Typebr),
    SBLE(Typebr),
    SBGT(Typebr),
    SBGE(Typebr),
    SLO(Typebr),
    SLS(Typebr),
    SLT(Typebr),
    SMI(Typebr),
    SNE(Typebr),
    SPL(Typebr),
    SRA(Typebr),
    SRN(Typebr),
    SSR(Typebr),
    SVC(Typebr),
    SBVS(Typebr),
    SEC(Type0),
    SEF(Type0),
    SEI(Type0),
    SEIF(Type0),
    SEV(Type0),
    SEX(Type0),
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
    SWI(Type0),
    SWI2(Type0),
    SWI3(Type0),
    SYNC(Type0),
    TFR(Typext),
    TSTA(Type0),
    TSTB(Type0),
    TST(Type2)
}

fn main() {
    println!("Hello, world!");
}

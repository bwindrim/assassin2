enum Type1 {
    BYTELOAD,
    BYTESTORE,
    WORDLOAD,
    WORDSTORE
}

enum Type2 {

}

enum Typeshift {

}

enum Typelea {

}

enum Typebr {

}

enum Typepspl {

}

enum Typecc {

}

enum Typesc {

}

enum Typext {

}

enum Typeswi {

}

enum Typemisc {

}

enum Instruction {
    ABX(Typemisc),
    ADCA(Type1),
    ADCB(Type1),
    ADDA(Type1),
    ADDB(Type1),
    ADDD(Type1),
    ANDA(Type1),
    ANDB(Type1),
    ANDCC(Typecc),
    ASLA(Typeshift),
    ASLB(Typeshift),
    ASRA(Typeshift),
    ASRB(Typeshift),
    ASR(Typeshift),
    BCC(Typebr),
    BCS(Typebr),
    BEQ(Typebr),
    BGE(Typebr),
    BGT(Typebr),
    BHI(Typebr),
    BHS(Typebr),
    BITA(Type1),
    BITB(Type1),
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
    CLC(Typesc),
    CLF(Typesc),
    CLI(Typesc),
    CLIF(Typesc),
    CLRA(Type2),
    CLRB(Type2),
    CLR(Type2),
    CLV(Typesc),
    CMPA(Type1),
    CMPB(Type1),
    CMPD(Type1),
    CMPS(Type1),
    CMPU(Type1),
    CMPX(Type1),
    CMPY(Type1),
    COMA(Type2),
    COMB(Type2),
    COM(Type2),
    CWAI(Typecc),
    DAA(Typemisc),
    DECA(Type2),
    DECB(Type2),
    DEC(Type2),
    EORA(Type1),
    EORB(Type1),
    EXG(Typext),
    INCA(Type2),
    INCB(Type2),
    INC(Type2),
    JMP(Type2),
    JSR(Type1),
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
    LDA(Type1),
    LDB(Type1),
    LDD(Type1),
    LDS(Type1),
    LDU(Type1),
    LDX(Type1),
    LDY(Type1),
    LEAS(Typelea),
    LEAU(Typelea),
    LEAX(Typelea),
    LEAY(Typelea),
    LSLA(Typeshift),
    LSLB(Typeshift),
    LSL(Typeshift),
    LSRA(Typeshift),
    LSRB(Typeshift),
    LSR(Typeshift),
    MUL(Typemisc),
    NEGA(Type2),
    NEGB(Type2),
    NEG(Type2),
    NOP(Typemisc),
    ORA(Type1),
    ORB(Type1),
    ORCC(Typecc),
    PSHU(Typepspl),
    PSHS(Typepspl),
    PULU(Typepspl),
    PULS(Typepspl),
    ROLA(Typeshift),
    ROLB(Typeshift),
    ROL(Typeshift),
    RORA(Typeshift),
    RORB(Typeshift),
    ROR(Typeshift),
    RTI(Typemisc),
    RTS(Typemisc),
    SBCA(Type1),
    SBCB(Type1),
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
    SEC(Typesc),
    SEF(Typesc),
    SEI(Typesc),
    SEIF(Typesc),
    SEV(Typesc),
    SEX(Typemisc),
    STA(Type1),
    STB(Type1),
    STD(Type1),
    STS(Type1),
    STU(Type1),
    STX(Type1),
    STY(Type1),
    SUBA(Type1),
    SUBB(Type1),
    SUBD(Type1),
    SWI(Typeswi),
    SWI2(Typeswi),
    SWI3(Typeswi),
    SYNC(Typemisc),
    TFR(Typext),
    TSTA(Type2),
    TSTB(Type2),
    TST(Type2)
}

fn main() {
    println!("Hello, world!");
}

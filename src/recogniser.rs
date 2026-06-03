use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::parser::{Token, tokenize};
use crate::representation::{
    Directive, Element, IndexedIndirect, Instruction, IntoBytes, Line, MemoryOperand,
    PushPullRegister, Stack, TfrExgRegister8, TfrExgRegister16, Type1, Type2, Typebr, Typecc,
    Typepspl, Typext,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;

// The Namelist struct is used to store the mapping of labels to their corresponding addresses in the assembled code.
#[derive(Debug)]
pub struct Recogniser {
    pc: u16,
    origin: Option<u16>,
    execute_address: Option<u16>,
    labels: HashMap<String, i32>,
}

impl Recogniser {
    pub fn new() -> Self {
        Recogniser {
            pc: 0,
            origin: None,
            execute_address: None,
            labels: HashMap::new(),
        }
    }

    fn insert(&mut self, label: String, value: i32) {
        self.labels.insert(label, value);
    }

    fn get(&self, label: &str) -> Option<&i32> {
        self.labels.get(label)
    }

    fn recognise(&mut self, tokens: &[Token]) -> Result<Option<Element>, String> {
        // This function will take a slice of tokens and attempt to recognise them as instructions or directives.
        // It will return a Result Element containing either the recognised instruction/directive
        // or an error if the tokens are not valid.

        let mut iter = tokens.iter(); // get an iterator over the tokens
        let first_token = iter.next(); // get the first token

        // If there are no tokens, or just a comment, then we do nothing but return success.
        // If there is a name at the start of the line then we hold onto it as a potential
        // label, but if there is an empty token instead then we proceed with no label.
        let label = match first_token {
            None => return Ok(None),                    // empty line
            Some(Token::Empty) => None,                 // no label
            Some(Token::Comment(_)) => return Ok(None), // comment-only line
            Some(Token::Name(name)) => Some(name),      // save the label
            _ => return Err("Unexpected token".to_string()),
        };

        // Check for a label, which will comprise an already-seen label followed by a colon.
        let mut next_token = iter.next();
        match next_token {
            None => return Err("Unexpected end of line".to_string()),
            Some(Token::Colon) => {
                next_token = iter.next(); // skip to the next token
                if let Some(name) = label {
                    // Define label.
                    self.insert(name.clone(), self.pc as i32); // insert the label into the namelist
                    println!("Label: {}", name);
                } else {
                    return Err("Unexpected : at start of line".to_string());
                }
            }
            _ => {} // other tokens will be handled below
        }

        // Main mnemonic recogniser.
        let element = match next_token {
            None => return Ok(None),
            Some(Token::Name(mnemonic)) => match mnemonic.to_uppercase().as_str() {
                "ORG" => match iter.next() {
                    None => return Err("Expected operand after ORG".to_string()),
                    Some(Token::Unsigned(value)) => {
                        if None != self.origin {
                            return Err("Multiple ORG directives".to_string());
                        }
                        self.origin = Some(*value);
                        self.pc = *value; // set the program counter to the specified value
                        Element::Directive(Directive::ORG(*value))
                    }
                    _ => return Err("Expected unsigned value after ORG".to_string()),
                },
                "EXEC" => match iter.next() {
                    None => return Err("Expected operand after EXEC".to_string()),
                    Some(Token::Unsigned(value)) => {
                        if None != self.execute_address {
                            return Err("Multiple EXEC directives".to_string());
                        }
                        self.execute_address = Some(*value);
                        Element::Directive(Directive::EXEC(*value))
                    }
                    _ => return Err("Expected unsigned value after EXEC".to_string()),
                },
                "EQU" => match iter.next() {
                    None => return Err("Expected operand after EQU".to_string()),
                    Some(Token::Unsigned(value)) => {
                        self.insert(
                            label
                                .ok_or("EQU directive must have a name".to_string())?
                                .clone(),
                            *value as i32,
                        );
                        Element::Directive(Directive::EQU(*value))
                    }
                    _ => return Err("Expected unsigned value after EQU".to_string()),
                },
                "DB" => {
                    let mut values = Vec::new();
                    // Use while let here, rather than a for loop, so that the iterator is
                    // still available after the loop to check for surplus tokens.
                    while let Some(token) = iter.next() {
                        match token {
                            // ToDo: make this stricter wrt. values and commas?
                            Token::Unsigned(value) => values.push(*value as u8),
                            Token::String(string) => values.extend(string.bytes()),
                            Token::Comma => continue, // allow commas between values
                            Token::Comment(_) => break,
                            _ => return Err("Expected unsigned value after DB".to_string()),
                        }
                    }
                    Element::Directive(Directive::DB(values))
                }
                "DW" => {
                    let mut values = Vec::new();
                    // Use while let here, rather than a for loop, so that the iterator is
                    // still available after the loop to check for surplus tokens.
                    while let Some(token) = iter.next() {
                        match token {
                            // ToDo: make this stricter wrt. values and commas?
                            Token::Unsigned(value) => values.push(*value),
                            Token::Comma => continue, // allow commas between values
                            Token::Comment(_) => break,
                            _ => return Err("Expected unsigned value after DW".to_string()),
                        }
                    }
                    Element::Directive(Directive::DW(values))
                }
                "DS" => match iter.next() {
                    // ToDo: check for unexpected tokens after the value?
                    None => return Err("Expected operand after DS".to_string()),
                    Some(Token::Unsigned(value)) => Element::Directive(Directive::DS(*value)),
                    _ => return Err("Expected unsigned value after DS".to_string()),
                },
                "ABX" => Element::Instruction(Instruction::ABX),
                "ADCA" => Element::Instruction(Instruction::ADCA(self.do_type1(&mut iter)?)),
                "ADCB" => Element::Instruction(Instruction::ADCB(self.do_type1(&mut iter)?)),
                "ADDA" => Element::Instruction(Instruction::ADDA(self.do_type1(&mut iter)?)),
                "ADDB" => Element::Instruction(Instruction::ADDB(self.do_type1(&mut iter)?)),
                "ADDD" => Element::Instruction(Instruction::ADDD(self.do_type1(&mut iter)?)),
                "ANDA" => Element::Instruction(Instruction::ANDA(self.do_type1(&mut iter)?)),
                "ANDB" => Element::Instruction(Instruction::ANDB(self.do_type1(&mut iter)?)),
                "ANDCC" => Element::Instruction(Instruction::ANDCC(self.do_typecc(&mut iter)?)),
                "ASLA" => Element::Instruction(Instruction::ASLA),
                "ASLB" => Element::Instruction(Instruction::ASLB),
                "ASL" => Element::Instruction(Instruction::ASL(self.do_type2(&mut iter)?)),
                "ASRA" => Element::Instruction(Instruction::ASRA),
                "ASRB" => Element::Instruction(Instruction::ASRB),
                "ASR" => Element::Instruction(Instruction::ASR(self.do_type2(&mut iter)?)),
                "BITA" => Element::Instruction(Instruction::BITA(self.do_type1(&mut iter)?)),
                "BITB" => Element::Instruction(Instruction::BITB(self.do_type1(&mut iter)?)),
                "BCC" => Element::Instruction(Instruction::BCC(self.do_typebr(&mut iter)?)),
                "BCS" => Element::Instruction(Instruction::BCS(self.do_typebr(&mut iter)?)),
                "BEQ" => Element::Instruction(Instruction::BEQ(self.do_typebr(&mut iter)?)),
                "BGE" => Element::Instruction(Instruction::BGE(self.do_typebr(&mut iter)?)),
                "BGT" => Element::Instruction(Instruction::BGT(self.do_typebr(&mut iter)?)),
                "BHI" => Element::Instruction(Instruction::BHI(self.do_typebr(&mut iter)?)),
                "BHS" => Element::Instruction(Instruction::BHS(self.do_typebr(&mut iter)?)),
                "BLE" => Element::Instruction(Instruction::BLE(self.do_typebr(&mut iter)?)),
                "BLO" => Element::Instruction(Instruction::BLO(self.do_typebr(&mut iter)?)),
                "BLS" => Element::Instruction(Instruction::BLS(self.do_typebr(&mut iter)?)),
                "BLT" => Element::Instruction(Instruction::BLT(self.do_typebr(&mut iter)?)),
                "BMI" => Element::Instruction(Instruction::BMI(self.do_typebr(&mut iter)?)),
                "BNE" => Element::Instruction(Instruction::BNE(self.do_typebr(&mut iter)?)),
                "BPL" => Element::Instruction(Instruction::BPL(self.do_typebr(&mut iter)?)),
                "BRA" => Element::Instruction(Instruction::BRA(self.do_typebr(&mut iter)?)),
                "BRN" => Element::Instruction(Instruction::BRN(self.do_typebr(&mut iter)?)),
                "BSR" => Element::Instruction(Instruction::BSR(self.do_typebr(&mut iter)?)),
                "BVC" => Element::Instruction(Instruction::BVC(self.do_typebr(&mut iter)?)),
                "BVS" => Element::Instruction(Instruction::BVS(self.do_typebr(&mut iter)?)),
                "CLC" => Element::Instruction(Instruction::CLC),
                "CLF" => Element::Instruction(Instruction::CLF),
                "CLI" => Element::Instruction(Instruction::CLI),
                "CLIF" => Element::Instruction(Instruction::CLIF),
                "CLRA" => Element::Instruction(Instruction::CLRA),
                "CLRB" => Element::Instruction(Instruction::CLRB),
                "CLR" => Element::Instruction(Instruction::CLR(self.do_type2(&mut iter)?)),
                "CMPA" => Element::Instruction(Instruction::CMPA(self.do_type1(&mut iter)?)),
                "CMPB" => Element::Instruction(Instruction::CMPB(self.do_type1(&mut iter)?)),
                "CMPD" => Element::Instruction(Instruction::CMPD(self.do_type1(&mut iter)?)),
                "CMPS" => Element::Instruction(Instruction::CMPS(self.do_type1(&mut iter)?)),
                "CMPU" => Element::Instruction(Instruction::CMPU(self.do_type1(&mut iter)?)),
                "CMPX" => Element::Instruction(Instruction::CMPX(self.do_type1(&mut iter)?)),
                "CMPY" => Element::Instruction(Instruction::CMPY(self.do_type1(&mut iter)?)),
                "CLV" => Element::Instruction(Instruction::CLV),
                "COMA" => Element::Instruction(Instruction::COMA),
                "COMB" => Element::Instruction(Instruction::COMB),
                "COM" => Element::Instruction(Instruction::COM(self.do_type2(&mut iter)?)),
                "CWAI" => Element::Instruction(Instruction::CWAI(self.do_typecc(&mut iter)?)),
                "DAA" => Element::Instruction(Instruction::DAA),
                "DECA" => Element::Instruction(Instruction::DECA),
                "DECB" => Element::Instruction(Instruction::DECB),
                "DEC" => Element::Instruction(Instruction::DEC(self.do_type2(&mut iter)?)),
                "EXG" => Element::Instruction(Instruction::EXG(self.do_typext(&mut iter)?)),
                "INCA" => Element::Instruction(Instruction::INCA),
                "INCB" => Element::Instruction(Instruction::INCB),
                "INC" => Element::Instruction(Instruction::INC(self.do_type2(&mut iter)?)),
                "JMP" => Element::Instruction(Instruction::JMP(self.do_type2(&mut iter)?)),
                "JSR" => Element::Instruction(Instruction::JSR(self.do_type2(&mut iter)?)),
                "LBCC" => Element::Instruction(Instruction::BCC(self.do_typelbr(&mut iter)?)),
                "LBCS" => Element::Instruction(Instruction::BCS(self.do_typelbr(&mut iter)?)),
                "LBEQ" => Element::Instruction(Instruction::BEQ(self.do_typelbr(&mut iter)?)),
                "LBGE" => Element::Instruction(Instruction::BGE(self.do_typelbr(&mut iter)?)),
                "LBGT" => Element::Instruction(Instruction::BGT(self.do_typelbr(&mut iter)?)),
                "LBHI" => Element::Instruction(Instruction::BHI(self.do_typelbr(&mut iter)?)),
                "LBHS" => Element::Instruction(Instruction::BHS(self.do_typelbr(&mut iter)?)),
                "LBLE" => Element::Instruction(Instruction::BLE(self.do_typelbr(&mut iter)?)),
                "LBLO" => Element::Instruction(Instruction::BLO(self.do_typelbr(&mut iter)?)),
                "LBLS" => Element::Instruction(Instruction::BLS(self.do_typelbr(&mut iter)?)),
                "LBLT" => Element::Instruction(Instruction::BLT(self.do_typelbr(&mut iter)?)),
                "LBMI" => Element::Instruction(Instruction::BMI(self.do_typelbr(&mut iter)?)),
                "LBNE" => Element::Instruction(Instruction::BNE(self.do_typelbr(&mut iter)?)),
                "LBPL" => Element::Instruction(Instruction::BPL(self.do_typelbr(&mut iter)?)),
                "LBRA" => Element::Instruction(Instruction::BRA(self.do_typelbr(&mut iter)?)),
                "LBRN" => Element::Instruction(Instruction::BRN(self.do_typelbr(&mut iter)?)),
                "LBSR" => Element::Instruction(Instruction::BSR(self.do_typelbr(&mut iter)?)),
                "LBVC" => Element::Instruction(Instruction::BVC(self.do_typelbr(&mut iter)?)),
                "LBVS" => Element::Instruction(Instruction::BVS(self.do_typelbr(&mut iter)?)),
                "LDA" => Element::Instruction(Instruction::LDA(self.do_type1(&mut iter)?)),
                "LDB" => Element::Instruction(Instruction::LDB(self.do_type1(&mut iter)?)),
                "LDD" => Element::Instruction(Instruction::LDD(self.do_type1(&mut iter)?)),
                "LDX" => Element::Instruction(Instruction::LDX(self.do_type1(&mut iter)?)),
                "LDY" => Element::Instruction(Instruction::LDY(self.do_type1(&mut iter)?)),
                "LDS" => Element::Instruction(Instruction::LDS(self.do_type1(&mut iter)?)),
                "LDU" => Element::Instruction(Instruction::LDU(self.do_type1(&mut iter)?)),
                "LEAX" => Element::Instruction(Instruction::LEAX(self.do_type2(&mut iter)?)),
                "LEAY" => Element::Instruction(Instruction::LEAY(self.do_type2(&mut iter)?)),
                "LEAU" => Element::Instruction(Instruction::LEAU(self.do_type2(&mut iter)?)),
                "LEAS" => Element::Instruction(Instruction::LEAS(self.do_type2(&mut iter)?)),
                "LSLA" => Element::Instruction(Instruction::LSLA),
                "LSLB" => Element::Instruction(Instruction::LSLB),
                "LSL" => Element::Instruction(Instruction::LSL(self.do_type2(&mut iter)?)),
                "LSRA" => Element::Instruction(Instruction::LSRA),
                "LSRB" => Element::Instruction(Instruction::LSRB),
                "LSR" => Element::Instruction(Instruction::LSR(self.do_type2(&mut iter)?)),
                "MUL" => Element::Instruction(Instruction::MUL),
                "NEGA" => Element::Instruction(Instruction::NEGA),
                "NEGB" => Element::Instruction(Instruction::NEGB),
                "NEG" => Element::Instruction(Instruction::NEG(self.do_type2(&mut iter)?)),
                "NOP" => Element::Instruction(Instruction::NOP),
                "ORA" => Element::Instruction(Instruction::ORA(self.do_type1(&mut iter)?)),
                "ORB" => Element::Instruction(Instruction::ORB(self.do_type1(&mut iter)?)),
                "ORCC" => Element::Instruction(Instruction::ORCC(self.do_typecc(&mut iter)?)),
                "PSHS" => {
                    Element::Instruction(Instruction::PSHS(self.do_typepspl(Stack::S, &mut iter)?))
                }
                "PSHU" => {
                    Element::Instruction(Instruction::PSHU(self.do_typepspl(Stack::U, &mut iter)?))
                }
                "PULS" => {
                    Element::Instruction(Instruction::PULS(self.do_typepspl(Stack::S, &mut iter)?))
                }
                "PULU" => {
                    Element::Instruction(Instruction::PULU(self.do_typepspl(Stack::U, &mut iter)?))
                }
                "ROLA" => Element::Instruction(Instruction::ROLA),
                "ROLB" => Element::Instruction(Instruction::ROLB),
                "ROL" => Element::Instruction(Instruction::ROL(self.do_type2(&mut iter)?)),
                "RORA" => Element::Instruction(Instruction::RORA),
                "RORB" => Element::Instruction(Instruction::RORB),
                "ROR" => Element::Instruction(Instruction::ROR(self.do_type2(&mut iter)?)),
                "RTI" => Element::Instruction(Instruction::RTI),
                "RTS" => Element::Instruction(Instruction::RTS),
                "SBCA" => Element::Instruction(Instruction::SBCA(self.do_type1(&mut iter)?)),
                "SBCB" => Element::Instruction(Instruction::SBCB(self.do_type1(&mut iter)?)),
                "SBCC" => Element::Instruction(Instruction::BCC(self.do_typesbr(&mut iter)?)),
                "SBCS" => Element::Instruction(Instruction::BCS(self.do_typesbr(&mut iter)?)),
                "SBEQ" => Element::Instruction(Instruction::BEQ(self.do_typesbr(&mut iter)?)),
                "SBGE" => Element::Instruction(Instruction::BGE(self.do_typesbr(&mut iter)?)),
                "SBGT" => Element::Instruction(Instruction::BGT(self.do_typesbr(&mut iter)?)),
                "SBHI" => Element::Instruction(Instruction::BHI(self.do_typesbr(&mut iter)?)),
                "SBHS" => Element::Instruction(Instruction::BHS(self.do_typesbr(&mut iter)?)),
                "SBLE" => Element::Instruction(Instruction::BLE(self.do_typesbr(&mut iter)?)),
                "SLO" => Element::Instruction(Instruction::BLO(self.do_typesbr(&mut iter)?)),
                "SLS" => Element::Instruction(Instruction::BLS(self.do_typesbr(&mut iter)?)),
                "SLT" => Element::Instruction(Instruction::BLT(self.do_typesbr(&mut iter)?)),
                "SBMI" => Element::Instruction(Instruction::BMI(self.do_typesbr(&mut iter)?)),
                "SBNE" => Element::Instruction(Instruction::BNE(self.do_typesbr(&mut iter)?)),
                "SBPL" => Element::Instruction(Instruction::BPL(self.do_typesbr(&mut iter)?)),
                "SBRA" => Element::Instruction(Instruction::BRA(self.do_typesbr(&mut iter)?)),
                "SBRN" => Element::Instruction(Instruction::BRN(self.do_typesbr(&mut iter)?)),
                "SBSR" => Element::Instruction(Instruction::BSR(self.do_typesbr(&mut iter)?)),
                "SBVC" => Element::Instruction(Instruction::BVC(self.do_typesbr(&mut iter)?)),
                "SBVS" => Element::Instruction(Instruction::BVS(self.do_typesbr(&mut iter)?)),
                "SEC" => Element::Instruction(Instruction::SEC),
                "SEF" => Element::Instruction(Instruction::SEF),
                "SEI" => Element::Instruction(Instruction::SEI),
                "SEIF" => Element::Instruction(Instruction::SEIF),
                "SEV" => Element::Instruction(Instruction::SEV),
                "SEX" => Element::Instruction(Instruction::SEX),
                "STA" => Element::Instruction(Instruction::STA(self.do_type2(&mut iter)?)),
                "STB" => Element::Instruction(Instruction::STB(self.do_type2(&mut iter)?)),
                "STD" => Element::Instruction(Instruction::STD(self.do_type2(&mut iter)?)),
                "STX" => Element::Instruction(Instruction::STX(self.do_type2(&mut iter)?)),
                "STY" => Element::Instruction(Instruction::STY(self.do_type2(&mut iter)?)),
                "STU" => Element::Instruction(Instruction::STU(self.do_type2(&mut iter)?)),
                "STS" => Element::Instruction(Instruction::STS(self.do_type2(&mut iter)?)),
                "SUBA" => Element::Instruction(Instruction::SUBA(self.do_type1(&mut iter)?)),
                "SUBB" => Element::Instruction(Instruction::SUBB(self.do_type1(&mut iter)?)),
                "SUBD" => Element::Instruction(Instruction::SUBD(self.do_type1(&mut iter)?)),
                "SWI" => Element::Instruction(Instruction::SWI),
                "SWI2" => Element::Instruction(Instruction::SWI2),
                "SWI3" => Element::Instruction(Instruction::SWI3),
                "SYNC" => Element::Instruction(Instruction::SYNC),
                "TFR" => Element::Instruction(Instruction::TFR(self.do_typext(&mut iter)?)),
                "TSTA" => Element::Instruction(Instruction::TSTA),
                "TSTB" => Element::Instruction(Instruction::TSTB),
                "TST" => Element::Instruction(Instruction::TST(self.do_type2(&mut iter)?)),

                _ => return Err("unknown mnemonic".to_string()),
            },
            _ => return Err("Unexpected token".to_string()),
        };
        if let Some(token) = iter.next() {
            match token {
                Token::Comment(_) => {} // allow a comment at the end of the line
                _ => return Err("Unexpected token(s) at end of line".to_string()),
            }
        }
        Ok(Some(element))
    }

    fn do_typecc(&mut self, tokens: &mut std::slice::Iter<Token>) -> Result<Typecc, String> {
        match tokens.next() {
            Some(Token::Hash) => match tokens.next() {
                Some(Token::Unsigned(value)) => {
                    if *value <= 255 {
                        Ok(Typecc { mask: *value as u8 })
                    } else {
                        Err("Invalid numeric value in condition mask".to_string())
                    }
                }
                _ => Err("Expected condition mask".to_string()),
            },
            _ => Err("Expected # before condition mask".to_string()),
        }
    }

    fn do_typext(&mut self, token: &mut std::slice::Iter<Token>) -> Result<Typext, String> {
        fn get_tfr_exg_register16(reg: &str) -> Result<TfrExgRegister16, String> {
            match reg {
                "D" | "d" => Ok(TfrExgRegister16::D),
                "X" | "x" => Ok(TfrExgRegister16::X),
                "Y" | "y" => Ok(TfrExgRegister16::Y),
                "U" | "u" => Ok(TfrExgRegister16::U),
                "S" | "s" => Ok(TfrExgRegister16::S),
                "PC" | "pc" => Ok(TfrExgRegister16::PC),
                _ => Err(format!("Invalid TFR/EXG register: {}", reg)),
            }
        }

        fn get_tfr_exg_register8(reg: &str) -> Result<TfrExgRegister8, String> {
            match reg {
                "A" | "a" => Ok(TfrExgRegister8::A),
                "B" | "b" => Ok(TfrExgRegister8::B),
                "CC" | "cc" => Ok(TfrExgRegister8::CC),
                "DP" | "dp" => Ok(TfrExgRegister8::DP),
                _ => Err(format!("Invalid TFR/EXG register: {}", reg)),
            }
        }

        if let Some(Token::Name(src)) = token.next() {
            if let Some(Token::Comma) = token.next() {
                if let Some(Token::Name(dst)) = token.next() {
                    if let Ok(src_reg) = get_tfr_exg_register8(src) {
                        let dst_reg = get_tfr_exg_register8(dst)?;
                        Ok(Typext::BYTE(src_reg, dst_reg))
                    } else if let Ok(src_reg) = get_tfr_exg_register16(src) {
                        let dst_reg = get_tfr_exg_register16(dst)?;
                        Ok(Typext::WORD(src_reg, dst_reg))
                    } else {
                        Err(format!("Invalid TFR/EXG source register: {}", src))
                    }
                } else {
                    Err("Expected destination register after comma in TFR/EXG".to_string())
                }
            } else {
                Err("Expected comma after source register in TFR/EXG".to_string())
            }
        } else {
            Err("Expected source register in TFR/EXG".to_string())
        }
    }

    fn do_typepspl(
        &mut self,
        stack: Stack,
        tokens: &mut std::slice::Iter<Token>,
    ) -> Result<Typepspl, String> {
        fn get_pspl_register(reg: &str, stack: &Stack) -> Result<PushPullRegister, String> {
            match reg {
                "A" | "a" => Ok(PushPullRegister::A),
                "B" | "b" => Ok(PushPullRegister::B),
                "X" | "x" => Ok(PushPullRegister::X),
                "Y" | "y" => Ok(PushPullRegister::Y),
                "U" | "u" => {
                    if *stack == Stack::S {
                        Ok(PushPullRegister::US)
                    } else {
                        Err("Can't push/pull U register on U stack".to_string())
                    }
                }
                "S" | "s" => {
                    if *stack == Stack::U {
                        Ok(PushPullRegister::US)
                    } else {
                        Err("Can't push/pull S register on S stack".to_string())
                    }
                }
                "PC" | "pc" => Ok(PushPullRegister::PC),
                "CC" | "cc" => Ok(PushPullRegister::CC),
                "DP" | "dp" => Ok(PushPullRegister::DP),
                _ => Err(format!("Invalid PSH/PUL register: {}", reg)),
            }
        }

        if let Some(Token::Name(reg_name)) = tokens.next() {
            let mut registers: HashSet<PushPullRegister> = HashSet::new();
            let reg = get_pspl_register(reg_name, &stack)?;
            registers.insert(reg);
            while let Some(Token::Comma) = tokens.next() {
                if let Some(Token::Name(reg_name)) = tokens.next() {
                    let reg = get_pspl_register(reg_name, &stack)?;
                    if !registers.insert(reg) {
                        return Err(format!("Duplicate register in PSH/PUL: {}", reg_name));
                    }
                } else {
                    return Err("Expected register name after comma in PSH/PUL".to_string());
                }
            }
            Ok(Typepspl { registers })
        } else {
            Err("Expected register name in PSH/PUL".to_string())
        }
    }

    fn do_indexed_indirect(
        tokens: &mut std::slice::Iter<Token>,
    ) -> Result<IndexedIndirect, String> {
        // Implementation for parsing indexed indirect addressing
        unimplemented!()
    }

    fn do_indexed(value: &u16, tokens: &mut std::slice::Iter<Token>) -> Option<IndexedIndirect> {
        // Implementation for parsing indexed indirect addressing
        unimplemented!()
    }

    fn parse_memory_operand(
        token: &Token,
        tokens: &mut std::slice::Iter<Token>,
    ) -> Result<MemoryOperand, String> {
        // Implementation for parsing a memory operand
        match token {
            Token::LessThan => {
                if let Some(Token::Unsigned(value)) = tokens.next() {
                    if *value <= 255 {
                        Ok(MemoryOperand::DIR(*value as u8))
                    } else {
                        Err("Direct page address too large for 8-bit offset".to_string())
                    }
                } else {
                    Err("Expected unsigned value after < in memory operand".to_string())
                }
            }
            Token::GreaterThan => {
                if let Some(Token::Unsigned(value)) = tokens.next() {
                    Ok(MemoryOperand::EXT(*value))
                } else {
                    Err("Expected unsigned value after > in memory operand".to_string())
                }
            }
            Token::Unsigned(value) => {
                //            if let Some(ind) = do_indexed(value, tokens) {
                //                Ok(MemoryOperand::IND(ind))
                //            } else {
                Ok(MemoryOperand::EXT(*value))
                //            }
            }
            Token::OpenBracket => {
                //            let ind = do_indexed_indirect(tokens)?;
                //            if let Some(Token::CloseBracket) = tokens.next() {
                //                Ok(MemoryOperand::IND(ind))
                //            } else {
                Err("Expected ] at end of indexed indirect operand".to_string())
                //            }
            }
            _ => Err("unexpected memory operand".to_string()),
        }
    }

    fn do_type1<T: IntoBytes + TryFrom<u16>>(
        &mut self,
        tokens: &mut std::slice::Iter<Token>,
    ) -> Result<Type1<T>, String> {
        match tokens.next() {
            Some(Token::Hash) => {
                if let Some(Token::Unsigned(value)) = tokens.next() {
                    // If T is only ever u8 or u16, this makes the mismatch go away.
                    // If the IntoBytes trait provided a conversion API we could use that instead of TryFrom<u16>.
                    let imm = T::try_from(*value).map_err(|_| {
                        "Immediate value not valid for Type1 operand type".to_string()
                    })?;
                    Ok(Type1::IMM(imm))
                } else {
                    Err("Expected unsigned value after # in immediate operand".to_string())
                }
            }
            Some(token) => Ok(Type1::MEM(Self::parse_memory_operand(token, tokens)?)),
            None => Err("Expected immediate or memory operand".to_string()),
        }
    }

    fn do_type2(&mut self, tokens: &mut std::slice::Iter<Token>) -> Result<Type2, String> {
        Ok(Type2 {
            operand: Self::parse_memory_operand(
                tokens.next().ok_or("Expected memory operand")?,
                tokens,
            )?,
        })
    }

    fn do_typebr(&mut self, tokens: &mut std::slice::Iter<Token>) -> Result<Typebr, String> {
        match tokens.next() {
            Some(Token::Unsigned(value)) => {
                if *value <= 127 {
                    Ok(Typebr::SHORT(*value as i8))
                } else if *value <= 32767 {
                    Ok(Typebr::LONG(*value as i16))
                } else {
                    Err("Branch target offset too large".to_string())
                }
            }
            Some(Token::Name(label)) => Ok(Typebr::UNRESOLVED(label.clone())),
            _ => Err("Expected unsigned value or label in branch operand".to_string()),
        }
    }

    fn do_typesbr(&mut self, tokens: &mut std::slice::Iter<Token>) -> Result<Typebr, String> {
        match tokens.next() {
            Some(Token::Unsigned(value)) => {
                if *value <= 127 {
                    Ok(Typebr::SHORT(*value as i8))
                } else {
                    Err("Branch target offset too large".to_string())
                }
            }
            _ => Err("Expected unsigned value in short branch operand".to_string()),
        }
    }

    fn do_typelbr(&mut self, tokens: &mut std::slice::Iter<Token>) -> Result<Typebr, String> {
        match tokens.next() {
            Some(Token::Unsigned(value)) => {
                if *value <= 32767 {
                    Ok(Typebr::LONG(*value as i16))
                } else {
                    Err("Branch target offset too large".to_string())
                }
            }
            Some(Token::Name(label)) => Ok(Typebr::UNRESOLVED(label.clone())),
            _ => Err("Expected unsigned value or label in branch operand".to_string()),
        }
    }

    pub fn recognise_line(&mut self, tokens: &[Token]) -> Result<Line, String> {
        // The line may end with a comment, so we check for that and separate it out if present.
        let (comment, tokens) = if let Some(Token::Comment(comment_string)) = tokens.last() {
            // The last token is a comment, so we take it out of the token slice and store it separately.
            (
                Some(comment_string.clone()), // store a copy of the comment string
                &tokens[..tokens.len() - 1], // discard the last Token from the slice (i.e. the comment)
            )
        } else {
            // No comment at the end of the line, or the line is empty, so we just proceed with the original tokens and no comment.
            (None, tokens)
        };
        // Now we attempt to recognise the remaining tokens as an instruction or directive, and return a Line
        // containing the recognised element and the comment (if any).
        match self.recognise(tokens) {
            Ok(None) => Ok(Line {
                element: Element::Directive(Directive::BLANK),
                comment,
            }),
            Ok(Some(element)) => Ok(Line { element, comment }),
            Err(e) => Err(e),
        }
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

    pub fn parse(&mut self, filename: &str) -> std::io::Result<()> {
        let path = env::current_dir()?;
        println!("The current directory is {}", path.display());

        // File <filename> must exist in the current path
        if let Ok(lines) = Self::read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.map_while(Result::ok) {
                println!("{}", line);
                {
                    let line: &str = &line;
                    let result = tokenize(line);
                    match result {
                        Ok(tokens) => {
                            println!("{:?}", tokens);
                            let line = self.recognise_line(&tokens);
                            match line {
                                Ok(line) => println!("Recognised line: {:?}", line),
                                Err(e) => println!("Recognition error: {}", e),
                            }
                        }
                        Err(e) => println!("Tokenizer error: {:?}", e),
                    }
                };
            }
        }
        Ok(())
    }
}

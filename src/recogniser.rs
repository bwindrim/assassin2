use crate::parser::Token;
use crate::representation::{
    Directive, Element, Instruction, Line, PushPullRegister, Stack, TfrExgRegister8,
    TfrExgRegister16, Typecc, Typepspl, Typext, Type2,
};
use std::collections::HashSet;

fn recognise(tokens: &[Token]) -> Result<Option<Element>, String> {
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
                // ToDo: define label
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
                Some(Token::Unsigned(value)) => Element::Directive(Directive::ORG(*value)),
                _ => return Err("Expected unsigned value after ORG".to_string()),
            },
            "EXEC" => match iter.next() {
                None => return Err("Expected operand after EXEC".to_string()),
                Some(Token::Unsigned(value)) => Element::Directive(Directive::EXEC(*value)),
                _ => return Err("Expected unsigned value after EXEC".to_string()),
            },
            "EQU" => match iter.next() {
                None => return Err("Expected operand after EQU".to_string()),
                Some(Token::Unsigned(value)) => Element::Directive(Directive::EQU(*value)),
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
            "ANDCC" => Element::Instruction(Instruction::ANDCC(do_typecc(&mut iter)?)),
            "ASLA" => Element::Instruction(Instruction::ASLA),
            "ASLB" => Element::Instruction(Instruction::ASLB),
            "ASL" => Element::Instruction(Instruction::ASL(do_type2(&mut iter)?)),
            "ASRA" => Element::Instruction(Instruction::ASRA),
            "ASRB" => Element::Instruction(Instruction::ASRB),
            "ASR" => Element::Instruction(Instruction::ASR(do_type2(&mut iter)?)),
            "CLC" => Element::Instruction(Instruction::CLC),
            "CLF" => Element::Instruction(Instruction::CLF),
            "CLI" => Element::Instruction(Instruction::CLI),
            "CLIF" => Element::Instruction(Instruction::CLIF),
            "CLRA" => Element::Instruction(Instruction::CLRA),
            "CLRB" => Element::Instruction(Instruction::CLRB),
            "CLR" => Element::Instruction(Instruction::CLR(do_type2(&mut iter)?)),
            "CLV" => Element::Instruction(Instruction::CLV),
            "COMA" => Element::Instruction(Instruction::COMA),
            "COMB" => Element::Instruction(Instruction::COMB),
            "COM" => Element::Instruction(Instruction::COM(do_type2(&mut iter)?)),
            "CWAI" => Element::Instruction(Instruction::CWAI(do_typecc(&mut iter)?)),
            "DAA" => Element::Instruction(Instruction::DAA),
            "DECA" => Element::Instruction(Instruction::DECA),
            "DECB" => Element::Instruction(Instruction::DECB),
            "DEC" => Element::Instruction(Instruction::DEC(do_type2(&mut iter)?)),
            "EXG" => Element::Instruction(Instruction::EXG(do_typext(&mut iter)?)),
            "INCA" => Element::Instruction(Instruction::INCA),
            "INCB" => Element::Instruction(Instruction::INCB),
            "INC" => Element::Instruction(Instruction::INC(do_type2(&mut iter)?)),
            "JMP" => Element::Instruction(Instruction::JMP(do_type2(&mut iter)?)),
            "JSR" => Element::Instruction(Instruction::JSR(do_type2(&mut iter)?)),
            "LEAX" => Element::Instruction(Instruction::LEAX(do_type2(&mut iter)?)),
            "LEAY" => Element::Instruction(Instruction::LEAY(do_type2(&mut iter)?)),
            "LEAU" => Element::Instruction(Instruction::LEAU(do_type2(&mut iter)?)),
            "LEAS" => Element::Instruction(Instruction::LEAS(do_type2(&mut iter)?)),
            "LSLA" => Element::Instruction(Instruction::LSLA),
            "LSLB" => Element::Instruction(Instruction::LSLB),
            "LSL" => Element::Instruction(Instruction::LSL(do_type2(&mut iter)?)),
            "LSRA" => Element::Instruction(Instruction::LSRA),
            "LSRB" => Element::Instruction(Instruction::LSRB),
            "LSR" => Element::Instruction(Instruction::LSR(do_type2(&mut iter)?)),
            "MUL" => Element::Instruction(Instruction::MUL),
            "NEGA" => Element::Instruction(Instruction::NEGA),
            "NEGB" => Element::Instruction(Instruction::NEGB),
            "NEG" => Element::Instruction(Instruction::NEG(do_type2(&mut iter)?)),
            "NOP" => Element::Instruction(Instruction::NOP),
            "ORCC" => Element::Instruction(Instruction::ORCC(do_typecc(&mut iter)?)),
            "PSHS" => Element::Instruction(Instruction::PSHS(do_typepspl(Stack::S, &mut iter)?)),
            "PSHU" => Element::Instruction(Instruction::PSHU(do_typepspl(Stack::U, &mut iter)?)),
            "PULS" => Element::Instruction(Instruction::PULS(do_typepspl(Stack::S, &mut iter)?)),
            "PULU" => Element::Instruction(Instruction::PULU(do_typepspl(Stack::U, &mut iter)?)),
            "ROLA" => Element::Instruction(Instruction::ROLA),
            "ROLB" => Element::Instruction(Instruction::ROLB),
            "ROL" => Element::Instruction(Instruction::ROL(do_type2(&mut iter)?)),
            "RORA" => Element::Instruction(Instruction::RORA),
            "RORB" => Element::Instruction(Instruction::RORB),
            "ROR" => Element::Instruction(Instruction::ROR(do_type2(&mut iter)?)),
            "RTI" => Element::Instruction(Instruction::RTI),
            "RTS" => Element::Instruction(Instruction::RTS),
            "SEC" => Element::Instruction(Instruction::SEC),
            "SEF" => Element::Instruction(Instruction::SEF),
            "SEI" => Element::Instruction(Instruction::SEI),
            "SEIF" => Element::Instruction(Instruction::SEIF),
            "SEV" => Element::Instruction(Instruction::SEV),
            "SEX" => Element::Instruction(Instruction::SEX),
            "STA" => Element::Instruction(Instruction::STA(do_type2(&mut iter)?)),
            "STB" => Element::Instruction(Instruction::STB(do_type2(&mut iter)?)),
            "STD" => Element::Instruction(Instruction::STD(do_type2(&mut iter)?)),
            "STX" => Element::Instruction(Instruction::STX(do_type2(&mut iter)?)),
            "STY" => Element::Instruction(Instruction::STY(do_type2(&mut iter)?)),
            "STU" => Element::Instruction(Instruction::STU(do_type2(&mut iter)?)),
            "STS" => Element::Instruction(Instruction::STS(do_type2(&mut iter)?)),
            "SWI" => Element::Instruction(Instruction::SWI),
            "SWI2" => Element::Instruction(Instruction::SWI2),
            "SWI3" => Element::Instruction(Instruction::SWI3),
            "SYNC" => Element::Instruction(Instruction::SYNC),
            "TFR" => Element::Instruction(Instruction::TFR(do_typext(&mut iter)?)),
            "TSTA" => Element::Instruction(Instruction::TSTA),
            "TSTB" => Element::Instruction(Instruction::TSTB),
            "TST" => Element::Instruction(Instruction::TST(do_type2(&mut iter)?)),

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

fn do_typecc(tokens: &mut std::slice::Iter<Token>) -> Result<Typecc, String> {
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

fn do_typext(token: &mut std::slice::Iter<Token>) -> Result<Typext, String> {
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

fn do_typepspl(stack: Stack, tokens: &mut std::slice::Iter<Token>) -> Result<Typepspl, String> {
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

pub fn do_type2(tokens: &mut std::slice::Iter<Token>) -> Result<Type2, String> {
    if let Some(Token::Unsigned(value)) = tokens.next() {
        Ok(Type2::EXT(*value))
    } else {
        Err("Expected unsigned value after # in Type2 operand".to_string())
    }
}

pub fn recognise_line(tokens: &[Token]) -> Result<Line, String> {
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
    match recognise(tokens) {
        Ok(None) => Ok(Line {
            element: Element::Directive(Directive::BLANK),
            comment,
        }),
        Ok(Some(element)) => Ok(Line { element, comment }),
        Err(e) => Err(e),
    }
}

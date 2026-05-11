use crate::parser::Token;
use crate::representation::{Directive, Element, Instruction, Segment};

pub fn recognise(tokens: &[Token]) -> Result<Option<Element>, String> {
    // This function will take a slice of tokens and attempt to recognise them as instructions or data.
    // It will return a Result containing either the recognised instruction/data or an error if the tokens are not valid.

    let mut iter = tokens.iter(); // get an iterator over the tokens
    let first_token = iter.next(); // get the first token

    // If there are no tokens, or just a comment, then we do nothing but return success.
    // If there is a name at the start of the line then we hold onto it as a potential
    // label, but if there is an empty token instead then we proceed with no label.
    let label = match first_token {
        None => return Ok(None),
        Some(Token::Empty) => None,
        Some(Token::Comment(_)) => return Ok(None),
        Some(Token::Name(name)) => Some(name),
        _ => return Err("Unexpected token".to_string()),
    };

    // Check for a lable, which will comprise an already-seen label followed by a colon.
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
            "ABX" => Element::Inst(Instruction::ABX),
            "ASLA" => Element::Inst(Instruction::ASLA),
            "ASLB" => Element::Inst(Instruction::ASLB),
            "ASRA" => Element::Inst(Instruction::ASRA),
            "ASRB" => Element::Inst(Instruction::ASRB),
            "CLC" => Element::Inst(Instruction::CLC),
            "CLF" => Element::Inst(Instruction::CLF),
            "CLI" => Element::Inst(Instruction::CLI),
            "CLIF" => Element::Inst(Instruction::CLIF),
            "CLRA" => Element::Inst(Instruction::CLRA),
            "CLRB" => Element::Inst(Instruction::CLRB),
            "CLV" => Element::Inst(Instruction::CLV),
            "COMA" => Element::Inst(Instruction::COMA),
            "COMB" => Element::Inst(Instruction::COMB),
            "DAA" => Element::Inst(Instruction::DAA),
            "DECA" => Element::Inst(Instruction::DECA),
            "DECB" => Element::Inst(Instruction::DECB),
            "INCA" => Element::Inst(Instruction::INCA),
            "INCB" => Element::Inst(Instruction::INCB),
            "LSLA" => Element::Inst(Instruction::LSLA),
            "LSLB" => Element::Inst(Instruction::LSLB),
            "LSRA" => Element::Inst(Instruction::LSRA),
            "LSRB" => Element::Inst(Instruction::LSRB),
            "MUL" => Element::Inst(Instruction::MUL),
            "NEGA" => Element::Inst(Instruction::NEGA),
            "NEGB" => Element::Inst(Instruction::NEGB),
            "NOP" => Element::Inst(Instruction::NOP),
            "ROLA" => Element::Inst(Instruction::ROLA),
            "ROLB" => Element::Inst(Instruction::ROLB),
            "RORA" => Element::Inst(Instruction::RORA),
            "RORB" => Element::Inst(Instruction::RORB),
            "RTI" => Element::Inst(Instruction::RTI),
            "RTS" => Element::Inst(Instruction::RTS),
            "SEC" => Element::Inst(Instruction::SEC),
            "SEF" => Element::Inst(Instruction::SEF),
            "SEI" => Element::Inst(Instruction::SEI),
            "SEIF" => Element::Inst(Instruction::SEIF),
            "SEV" => Element::Inst(Instruction::SEV),
            "SEX" => Element::Inst(Instruction::SEX),
            "SWI" => Element::Inst(Instruction::SWI),
            "SWI2" => Element::Inst(Instruction::SWI2),
            "SWI3" => Element::Inst(Instruction::SWI3),
            "SYNC" => Element::Inst(Instruction::SYNC),
            "TSTA" => Element::Inst(Instruction::TSTA),
            "TSTB" => Element::Inst(Instruction::TSTB),

            _ => return Err("unknown mnemonic".to_string()),
        },
        _ => return Err("Unexpected token".to_string()),
    };
    Ok(Some(element))
}

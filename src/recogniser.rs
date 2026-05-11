use crate::parser::Token;
use crate::representation::{Directive, Element, Instruction};

pub fn recognise(tokens: &[Token]) -> Result<Option<Element>, String> {
    // This function will take a slice of tokens and attempt to recognise them as instructions or data.
    // It will return a Result containing either the recognised instruction/data or an error if the tokens are not valid.

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
                for token in iter {
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
                for token in iter {
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
            "DS" => {
                let operand_token = iter.next().ok_or("Expected operand after DS")?;
                match operand_token {
                    Token::Unsigned(value) => Element::Directive(Directive::DS(*value)),
                    _ => return Err("Expected unsigned value after DS".to_string()),
                }
            }
            "ABX" => Element::Instruction(Instruction::ABX),
            "ASLA" => Element::Instruction(Instruction::ASLA),
            "ASLB" => Element::Instruction(Instruction::ASLB),
            "ASRA" => Element::Instruction(Instruction::ASRA),
            "ASRB" => Element::Instruction(Instruction::ASRB),
            "CLC" => Element::Instruction(Instruction::CLC),
            "CLF" => Element::Instruction(Instruction::CLF),
            "CLI" => Element::Instruction(Instruction::CLI),
            "CLIF" => Element::Instruction(Instruction::CLIF),
            "CLRA" => Element::Instruction(Instruction::CLRA),
            "CLRB" => Element::Instruction(Instruction::CLRB),
            "CLV" => Element::Instruction(Instruction::CLV),
            "COMA" => Element::Instruction(Instruction::COMA),
            "COMB" => Element::Instruction(Instruction::COMB),
            "DAA" => Element::Instruction(Instruction::DAA),
            "DECA" => Element::Instruction(Instruction::DECA),
            "DECB" => Element::Instruction(Instruction::DECB),
            "INCA" => Element::Instruction(Instruction::INCA),
            "INCB" => Element::Instruction(Instruction::INCB),
            "LSLA" => Element::Instruction(Instruction::LSLA),
            "LSLB" => Element::Instruction(Instruction::LSLB),
            "LSRA" => Element::Instruction(Instruction::LSRA),
            "LSRB" => Element::Instruction(Instruction::LSRB),
            "MUL" => Element::Instruction(Instruction::MUL),
            "NEGA" => Element::Instruction(Instruction::NEGA),
            "NEGB" => Element::Instruction(Instruction::NEGB),
            "NOP" => Element::Instruction(Instruction::NOP),
            "ROLA" => Element::Instruction(Instruction::ROLA),
            "ROLB" => Element::Instruction(Instruction::ROLB),
            "RORA" => Element::Instruction(Instruction::RORA),
            "RORB" => Element::Instruction(Instruction::RORB),
            "RTI" => Element::Instruction(Instruction::RTI),
            "RTS" => Element::Instruction(Instruction::RTS),
            "SEC" => Element::Instruction(Instruction::SEC),
            "SEF" => Element::Instruction(Instruction::SEF),
            "SEI" => Element::Instruction(Instruction::SEI),
            "SEIF" => Element::Instruction(Instruction::SEIF),
            "SEV" => Element::Instruction(Instruction::SEV),
            "SEX" => Element::Instruction(Instruction::SEX),
            "SWI" => Element::Instruction(Instruction::SWI),
            "SWI2" => Element::Instruction(Instruction::SWI2),
            "SWI3" => Element::Instruction(Instruction::SWI3),
            "SYNC" => Element::Instruction(Instruction::SYNC),
            "TSTA" => Element::Instruction(Instruction::TSTA),
            "TSTB" => Element::Instruction(Instruction::TSTB),

            _ => return Err("unknown mnemonic".to_string()),
        },
        _ => return Err("Unexpected token".to_string()),
    };
    Ok(Some(element))
}

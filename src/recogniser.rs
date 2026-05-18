use crate::parser::Token;
use crate::representation::{Directive, Element, Instruction, Line, Typecc};

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
            "CWAI" => Element::Instruction(Instruction::CWAI(do_typecc(&mut iter)?)),
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
            "ORCC" => Element::Instruction(Instruction::ORCC(do_typecc(&mut iter)?)),
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
    // ToDo: check for unexpected tokens remaining
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

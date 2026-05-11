use crate::parser::Token;
use crate::representation::{Data, Element, Instruction, Segment};

pub fn recognise(tokens: &[Token]) -> Result<Option<Element>, String> {
    // This function will take a slice of tokens and attempt to recognise them as instructions or data.
    // It will return a Result containing either the recognised instruction/data or an error if the tokens are not valid.

    let mut iter = tokens.iter();
    let first_token = iter.next();
    let label =  match first_token {
        None => return Ok(None),
        Some(Token::Empty) => None,
        Some(Token::Comment(_)) => return Ok(None),
        Some(Token::Name(name)) => Some(name),
        _ => return Err("Unexpected token".to_string()),
    };

    let mut next_token = iter.next();
    match next_token {
        None => return Err("Unexpected name".to_string()),
        Some(Token::Colon) => {
            next_token = iter.next();
            if let Some(name) = label {
                // ToDo: define label
                println!("Label: {}", name);
            } else {
                return Err("Unexpected : at start of line".to_string())
            }
        },
        _ => {} // other tokens will be handled below
    }

    Ok(None)
}
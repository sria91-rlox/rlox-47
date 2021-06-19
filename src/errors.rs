use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum LoxError {
    UnexpectedCharacter(usize, char),
    UnterminatedString
}

impl Error for LoxError {}

// TODO: Get better error display
impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem lexing")
    }
}

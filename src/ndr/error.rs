use crate::ndr::Token;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum NdrError {
    UnexpectedToken { token: Token },
    UnexpectedEOF,
}

impl Display for NdrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UnexpectedToken { token } => {
                write!(f, "Token inesperado: {}", token.value)
            }
            Self::UnexpectedEOF => {
                write!(f, "EOF inesperado.")
            }
        }
    }
}

impl Error for NdrError {}

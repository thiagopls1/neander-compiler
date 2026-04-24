use crate::ndr::Token;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum NdrError {
    UnexpectedToken { token: Token },
}

impl Display for NdrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UnexpectedToken { token } => {
                write!(f, "Token inesperado: {}", token.value)
            }
        }
    }
}

impl Error for NdrError {}

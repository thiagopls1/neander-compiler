use super::frontend::Token;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum NdrError {
    UnexpectedToken { token: Token },
    UnexpectedEOF,
    DuplicateLabel { label: String },
    InvalidLabelValue { label: String, value: String },
    UndefinedLabel { label: String },
    InvalidOperand { operand: String },
}

impl Display for NdrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UnexpectedToken { token } => {
                write!(f, "Token inesperado: {:?}", token.value)
            }
            Self::UnexpectedEOF => {
                write!(f, "EOF inesperado.")
            }
            Self::DuplicateLabel { label } => {
                write!(f, "Símbolo duplicado encontrado: {}", label)
            }
            Self::UndefinedLabel { label } => {
                write!(f, "Rótulo não não definido: {}", label)
            }
            Self::InvalidLabelValue { label, value } => {
                write!(f, "Valor inválido para o rótulo {}: {}", label, value)
            }
            Self::InvalidOperand { operand } => {
                write!(f, "Operando inválido: {}", operand)
            }
        }
    }
}

impl Error for NdrError {}

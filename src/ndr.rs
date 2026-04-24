use crate::ndr::error::NdrError;

pub mod error;

#[derive(Debug)]
pub enum TokenKind {
    ProgramStart,    // PROGRAMA
    ProgramEnd,      // FIM_PROGRAMA
    Comment,         // # Comment
    DeclareVariable, // VAR A
    Variable,        // A,B,C...
    AssignVariable,  // A = B
    NewLine,         // \n
    Number,          // 1,2,3,4
    Sum,             // +
    Minus,           // -
    Mult,            // *
    Div,             //
    Unknown,         // Unexpected Token
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    pub fn new(value: String, kind: TokenKind) -> Token {
        return Token { kind, value };
    }
}

fn is_variable(token: &str) -> bool {
    let mut chars = token.chars();

    match chars.next() {
        Some(c) if c.is_alphabetic() => chars.all(|c| c.is_alphanumeric()),
        _ => false,
    }
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, NdrError> {
    let mut tokens: Vec<Token> = vec![];
    for line in s.lines() {
        for word in line.split_whitespace() {
            match word {
                "PROGRAMA" => tokens.push(Token::new(String::from(word), TokenKind::ProgramStart)),
                "FIM_PROGRAMA" => {
                    tokens.push(Token::new(String::from(word), TokenKind::ProgramEnd))
                }
                "#" => {
                    tokens.push(Token::new(String::from(word), TokenKind::Comment));
                    break;
                }
                "VAR" => tokens.push(Token::new(String::from(word), TokenKind::DeclareVariable)),
                "=" => tokens.push(Token::new(String::from(word), TokenKind::AssignVariable)),
                "+" => tokens.push(Token::new(String::from(word), TokenKind::Sum)),
                "-" => tokens.push(Token::new(String::from(word), TokenKind::Minus)),
                "*" => tokens.push(Token::new(String::from(word), TokenKind::Mult)),
                "/" => tokens.push(Token::new(String::from(word), TokenKind::Div)),
                _ if word.parse::<i32>().is_ok() => {
                    tokens.push(Token::new(String::from(word), TokenKind::Number))
                }
                _ if is_variable(word) => {
                    tokens.push(Token::new(String::from(word), TokenKind::Variable))
                }
                _ => {
                    let unexpected_token = Token::new(String::from(word), TokenKind::Unknown);
                    return Err(NdrError::UnexpectedToken {
                        token: unexpected_token,
                    });
                }
            }
        }

        tokens.push(Token::new(String::from("\n"), TokenKind::NewLine));
    }

    Ok(tokens)
}

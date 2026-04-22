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
    Div,             // /
}

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    value: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(value: &'a str, kind: TokenKind) -> Token<'a> {
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

pub fn tokenize(s: &str) -> Result<Vec<Token>, ()> {
    let mut tokens: Vec<Token> = vec![];
    for line in s.lines() {
        for word in line.split_whitespace() {
            match word {
                "PROGRAMA" => tokens.push(Token::new(word, TokenKind::ProgramStart)),
                "FIM_PROGRAMA" => tokens.push(Token::new(word, TokenKind::ProgramEnd)),
                "#" => {
                    tokens.push(Token::new(word, TokenKind::Comment));
                    break;
                }
                "VAR" => tokens.push(Token::new(word, TokenKind::DeclareVariable)),
                "=" => tokens.push(Token::new(word, TokenKind::AssignVariable)),
                "+" => tokens.push(Token::new(word, TokenKind::Sum)),
                "-" => tokens.push(Token::new(word, TokenKind::Minus)),
                "*" => tokens.push(Token::new(word, TokenKind::Mult)),
                "/" => tokens.push(Token::new(word, TokenKind::Div)),
                _ if word.parse::<i32>().is_ok() => {
                    tokens.push(Token::new(word, TokenKind::Number))
                }
                _ if is_variable(word) => tokens.push(Token::new(word, TokenKind::Variable)),
                _ => return Err(()),
            }
        }

        tokens.push(Token::new("\n", TokenKind::NewLine));
    }

    Ok(tokens)
}

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
pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    pub fn new(value: &str, kind: TokenKind) -> Token {
        return Token {
            kind,
            value: String::from(value),
        };
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
    let code_lines: Vec<&str> = s.lines().collect();

    for line in code_lines {
        let line_words: Vec<&str> = line.split_whitespace().collect();

        let mut comment_flag = false;
        for word in line_words {
            if comment_flag {
                continue;
            }

            match word {
                "PROGRAMA" => tokens.push(Token::new(word, TokenKind::ProgramStart)),
                "FIM_PROGRAMA" => tokens.push(Token::new(word, TokenKind::ProgramEnd)),
                "#" => {
                    tokens.push(Token::new(word, TokenKind::Comment));
                    comment_flag = true;
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
                _ => continue,
            }
        }

        tokens.push(Token::new("\n", TokenKind::NewLine));
    }

    return Ok(tokens);
}

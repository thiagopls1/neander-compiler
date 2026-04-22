#[derive(Debug)]
pub enum NeanderCodeToken {
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

fn is_variable(token: &str) -> bool {
    let mut chars = token.chars();

    match chars.next() {
        Some(c) if c.is_alphabetic() => chars.all(|c| c.is_alphanumeric()),
        _ => false,
    }
}

pub fn gen_neander_code_tokens(s: &str) -> Result<Vec<NeanderCodeToken>, ()> {
    let mut tokens: Vec<NeanderCodeToken> = vec![];
    let code_lines: Vec<&str> = s.lines().collect();

    for line in code_lines {
        let line_words: Vec<&str> = line.split_whitespace().collect();

        let mut comment_flag = false;
        for word in line_words {
            if comment_flag {
                continue;
            }

            match word {
                "PROGRAMA" => tokens.push(NeanderCodeToken::ProgramStart),
                "FIM_PROGRAMA" => tokens.push(NeanderCodeToken::ProgramEnd),
                "#" => {
                    tokens.push(NeanderCodeToken::Comment);
                    comment_flag = true;
                }
                "VAR" => tokens.push(NeanderCodeToken::DeclareVariable),
                "=" => tokens.push(NeanderCodeToken::AssignVariable),
                "+" => tokens.push(NeanderCodeToken::Sum),
                "-" => tokens.push(NeanderCodeToken::Minus),
                "*" => tokens.push(NeanderCodeToken::Mult),
                "/" => tokens.push(NeanderCodeToken::Div),
                _ if word.parse::<i32>().is_ok() => tokens.push(NeanderCodeToken::Number),
                _ if is_variable(word) => tokens.push(NeanderCodeToken::Variable),
                _ => continue,
            }
        }

        tokens.push(NeanderCodeToken::NewLine);
    }

    return Ok(tokens);
}

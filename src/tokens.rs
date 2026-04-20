pub enum NeanderCodeToken {
    ProgramStart,    // PROGRAMA
    ProgramEnd,      // FIM_PROGRAMA
    Commentary,      // # Comment
    DeclareVariable, // VAR A
    AssignVariable,  // A = B
    // Operation Tokens
    Sum,   // +
    Minus, // -
    Mult,  // *
    Div,   // /
}

pub enum NeanderAsmToken {}

pub fn gen_neander_code_tokens(s: &str) -> Result<Vec<NeanderCodeToken>, ()> {
    let tokens = vec![NeanderCodeToken::ProgramStart, NeanderCodeToken::ProgramEnd];

    for _c in s.chars() {}

    return Ok(tokens);
}

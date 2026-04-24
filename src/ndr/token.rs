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
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(value: String, kind: TokenKind) -> Token {
        return Token { kind, value };
    }
}

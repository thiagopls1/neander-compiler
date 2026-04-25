#[derive(Debug, Clone)]
pub enum Mnemonic {
    NOP = 0x00,
    STA = 0x10,
    LDA = 0x20,
    ADD = 0x30,
    OR = 0x40,
    AND = 0x50,
    NOT = 0x60,
    JMP = 0x80,
    JN = 0x90, // ← estava faltando
    JZ = 0xA0,
    HLT = 0xF0,
}

impl Mnemonic {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "NOP" => Some(Self::NOP),
            "STA" => Some(Self::STA),
            "LDA" => Some(Self::LDA),
            "ADD" => Some(Self::ADD),
            "OR" => Some(Self::OR),
            "AND" => Some(Self::AND),
            "NOT" => Some(Self::NOT),
            "JMP" => Some(Self::JMP),
            "JZ" => Some(Self::JZ),
            "HLT" => Some(Self::HLT),
            _ => None,
        }
    }

    pub fn opcode(self) -> u8 {
        self as u8
    }

    pub fn requires_operand(&self) -> bool {
        match self {
            Self::NOT | Self::HLT | Self::NOP => false,
            _ => true,
        }
    }
}

pub enum Mnemonic {
    NOP = 0x0,
    STA = 0x10,
    LDA = 0x20,
    ADD = 0x30,
    OR = 0x50,
    AND = 0x60,
    NOT = 0x70,
    JMP = 0x80,
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

    pub fn has_operand(self) -> bool {
        matches!(
            self,
            Self::STA | Self::LDA | Self::ADD | Self::OR | Self::AND | Self::JMP | Self::JZ
        )
    }
}

use super::assembler::{Mnemonic, mem::NeanderMemory};
use crate::error::NdrError;

pub struct NeanderEmulator {
    pub pc: u8,  // Program Counter
    pub acc: u8, // Acumulador
    pub z: bool, // Flag Zero
    pub n: bool, // Flag Negativo
    pub halted: bool,

    pub memory: NeanderMemory,
}

impl NeanderEmulator {
    pub fn new(memory: NeanderMemory) -> Self {
        Self {
            pc: 0,
            acc: 0,
            z: false,
            n: false,
            halted: false,
            memory,
        }
    }

    pub fn run(&mut self) -> Result<(), NdrError> {
        while !self.halted {
            self.step().map_err(|err| err)?;
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), NdrError> {
        if self.halted {
            return Ok(());
        }

        let opcode = self.memory.data[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);

        let mnemonic = Mnemonic::from_opcode(opcode).map_err(|err| err)?;

        match mnemonic {
            Mnemonic::NOP => {}
            Mnemonic::STA => {
                let addr = self.fetch_operand();
                self.memory.data[addr as usize] = self.acc;
            }

            Mnemonic::LDA => {
                let addr = self.fetch_operand();
                self.acc = self.memory.data[addr as usize];
                self.update_flags();
            }

            Mnemonic::ADD => {
                let addr = self.fetch_operand();
                let value = self.memory.data[addr as usize];
                self.acc = self.acc.wrapping_add(value);
                self.update_flags();
            }

            Mnemonic::OR => {
                let addr = self.fetch_operand();
                self.acc |= self.memory.data[addr as usize];
                self.update_flags();
            }

            Mnemonic::AND => {
                let addr = self.fetch_operand();
                self.acc &= self.memory.data[addr as usize];
                self.update_flags();
            }

            Mnemonic::NOT => {
                self.acc = !self.acc;
                self.update_flags();
            }

            Mnemonic::JMP => {
                let addr = self.fetch_operand();
                self.pc = addr;
            }

            Mnemonic::JN => {
                let addr = self.fetch_operand();
                if self.n {
                    self.pc = addr;
                }
            }

            Mnemonic::JZ => {
                let addr = self.fetch_operand();
                if self.z {
                    self.pc = addr;
                }
            }

            Mnemonic::HLT => {
                self.halted = true;
            }
        }

        println!(
            "PC: {:03} | ACC: {:03} | INST: {:?}",
            self.pc, self.acc, mnemonic
        );
        Ok(())
    }

    fn fetch_operand(&mut self) -> u8 {
        let value = self.memory.data[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        value
    }

    fn update_flags(&mut self) {
        self.z = self.acc == 0;
        self.n = (self.acc & 0x80) != 0;
    }
}

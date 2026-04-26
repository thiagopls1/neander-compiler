use std::{fs, io};

use crate::error::NdrError;

pub struct NeanderMemory {
    pub data: Vec<u8>, // sempre 256 bytes
}

impl NeanderMemory {
    pub fn new(data: Vec<u8>) -> Result<Self, NdrError> {
        if data.len() != 256 {
            return Err(NdrError::InvalidMemSize { size: data.len() });
        }

        Ok(Self { data })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + 512);

        bytes.extend_from_slice(&[0x03, b'N', b'D', b'R']);
        for &b in &self.data {
            let word = b as u16;
            bytes.extend_from_slice(&word.to_le_bytes());
        }

        bytes
    }

    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let bytes = self.to_bytes();
        fs::write(path, bytes)
    }
}

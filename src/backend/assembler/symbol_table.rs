use crate::error::NdrError;
use std::collections::HashMap;

pub struct LabelsTable {
    pub table: HashMap<String, u8>,
}

impl LabelsTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, addr: u8) -> Result<(), NdrError> {
        if self.table.contains_key(&name) {
            return Err(NdrError::DuplicateLabel { label: name });
        }
        self.table.insert(name, addr);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<u8> {
        self.table.get(name).copied()
    }
}

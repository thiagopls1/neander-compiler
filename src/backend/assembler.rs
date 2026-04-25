pub mod mem;
pub mod mnemonic;
pub mod symbol_table;

pub use mnemonic::Mnemonic;
pub use symbol_table::LabelsTable;

use crate::error::NdrError;
use mem::NeanderMemory;
use std::collections::HashMap;
use std::str::Lines;

pub fn build(assembly: &str) -> Result<NeanderMemory, NdrError> {
    // Primeira passagem, na qual retorna uma tabela Rótulo -> Endereço e outra Endereço -> Valor
    // Caso encontre algo inválido, retorna NdrError
    validate_text_section(assembly.lines())?;
    let (labels, adresses) = build_labels_table(assembly.lines())?;

    // Segunda passagem, na qual ocorre a construção do .mem
    let byte_code = build_bytecode(assembly, &labels, &adresses)?;

    Ok(NeanderMemory::new(byte_code))
}

fn build_labels_table(lines: Lines) -> Result<(LabelsTable, HashMap<u8, u8>), NdrError> {
    let mut labels = LabelsTable::new();
    let mut memory_init = HashMap::new();

    let mut data_addr: u8 = 255;
    let mut in_data = false;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            ".DATA" => {
                in_data = true;
                continue;
            }
            ".TEXT" => {
                in_data = false;
                continue;
            }
            _ => {}
        }

        if in_data {
            // A DATA 5
            if parts.len() >= 3 && parts[1] == "DATA" {
                let label = parts[0].to_string();
                let value = parts[2]
                    .parse::<u8>()
                    .map_err(|_| NdrError::InvalidLabelValue {
                        label: label.clone(),
                        value: parts[2].to_string(),
                    })?;

                labels.insert(label, data_addr)?;
                memory_init.insert(data_addr, value);

                data_addr -= 1;
            }
        }
    }

    Ok((labels, memory_init))
}

fn build_bytecode(
    assembly: &str,
    labels: &LabelsTable,
    data_values: &HashMap<u8, u8>,
) -> Result<Vec<u8>, NdrError> {
    let mut memory = vec![0u8; 256];
    let mut entry_point: Option<u8> = None;

    // =========================
    // aplica .DATA
    // =========================
    for (addr, value) in data_values {
        memory[*addr as usize] = *value;
    }

    // =========================
    // .TEXT
    // =========================
    let mut in_text = false;
    let mut pc: u8 = 0;

    for line in assembly.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            ".TEXT" => {
                in_text = true;
                continue;
            }
            ".DATA" => {
                in_text = false;
                continue;
            }
            _ => {}
        }

        if !in_text {
            continue;
        }

        // ===== .ORG =====
        if parts[0] == ".ORG" {
            if parts.len() < 2 {
                return Err(NdrError::UnexpectedEOF);
            }

            let org = parts[1]
                .parse::<u8>()
                .map_err(|_| NdrError::InvalidOperand {
                    operand: parts[1].to_string(),
                })?;

            if entry_point.is_none() {
                entry_point = Some(org);
            }

            pc = org;
            continue;
        }

        // ignora outras diretivas
        if parts[0].starts_with('.') {
            continue;
        }

        // ===== instrução =====
        let mnemonic = Mnemonic::from_str(parts[0]).ok_or(NdrError::InvalidInstruction {
            instruction: parts[0].to_string(),
        })?;

        // escreve opcode
        memory[pc as usize] = mnemonic.clone().opcode();
        pc = pc.wrapping_add(1);

        // escreve operando
        if mnemonic.requires_operand() {
            if parts.len() < 2 {
                return Err(NdrError::MissingOperand {
                    instruction: parts[0].to_string(),
                });
            }

            let operand = resolve_operand(parts[1], labels)?;
            memory[pc as usize] = operand;
            pc = pc.wrapping_add(1);
        }
    }

    if let Some(entry) = entry_point {
        if entry != 0 {
            memory[0] = Mnemonic::JMP as u8;
            memory[1] = entry;
        }
    }

    Ok(memory)
}

fn validate_text_section(lines: Lines) -> Result<(), NdrError> {
    let mut in_text = false;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            ".TEXT" => {
                in_text = true;
                continue;
            }
            ".DATA" => {
                in_text = false;
                continue;
            }
            _ => {}
        }

        if !in_text {
            continue;
        }

        if parts[0].starts_with('.') {
            continue;
        }

        // ===== valida mnemonic =====
        let mnemonic = Mnemonic::from_str(parts[0]).ok_or(NdrError::InvalidInstruction {
            instruction: parts[0].to_string(),
        })?;

        // ===== valida operandos =====
        if mnemonic.requires_operand() {
            if parts.len() < 2 {
                return Err(NdrError::MissingOperand {
                    instruction: parts[0].to_string(),
                });
            }

            validate_operand_format(parts[1])?;
        } else {
            if parts.len() > 1 {
                return Err(NdrError::UnexpectedOperand {
                    instruction: parts[0].to_string(),
                    operand: parts[1].to_string(),
                });
            }
        }
    }

    Ok(())
}

fn validate_operand_format(op: &str) -> Result<(), NdrError> {
    // Rótulo
    if op.starts_with('[') && op.ends_with(']') {
        let name = &op[1..op.len() - 1];

        if name.is_empty() {
            return Err(NdrError::InvalidOperand {
                operand: op.to_string(),
            });
        }

        return Ok(());
    }

    // Número
    if op.parse::<u8>().is_ok() {
        return Ok(());
    }

    Err(NdrError::InvalidOperand {
        operand: op.to_string(),
    })
}

fn resolve_operand(op: &str, table: &LabelsTable) -> Result<u8, NdrError> {
    // É um rótulo, então buscamos na tabela
    if op.starts_with('[') && op.ends_with(']') {
        let name = &op[1..op.len() - 1];

        return table.get(name).ok_or(NdrError::UndefinedLabel {
            label: String::from(name),
        });
    }

    // Se não for, validamos se é um u8
    op.parse::<u8>().map_err(|_| NdrError::InvalidOperand {
        operand: String::from(op),
    })
}

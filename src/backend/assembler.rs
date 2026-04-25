pub mod mnemonic;
pub mod symbol_table;

pub use mnemonic::Mnemonic;
pub use symbol_table::LabelsTable;

use crate::error::NdrError;
use std::collections::HashMap;
use std::str::Lines;

pub fn build(assembly: &str) -> Result<(), NdrError> {
    // Primeira passagem, na qual retorna uma tabela Rótulo -> Endereço e outra Endereço -> Valor
    // Caso encontre algo inválido, retorna NdrError
    let (labels, adresses) = build_labels_table(assembly.lines()).map_err(|err| err)?;

    println!("Tabela de rótulos:");
    println!("{:?}", labels.table);
    println!("Tabela de endereços:");
    println!("{:?}", adresses);

    Ok(())
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

fn _resolve_operand(op: &str, table: &LabelsTable) -> Result<u8, NdrError> {
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

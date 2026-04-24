pub mod cli;
pub mod logging;
pub mod ndr;

use std::{fs, io::ErrorKind, process};

use clap::Parser;
use cli::Args;
use colored::*;
fn main() {
    let args = Args::parse();

    log_info!("Lendo arquivo {}...", args.file_path);
    let ndr_code = match fs::read_to_string(&args.file_path) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                log_error!("Arquivo {} não encontrado!", args.file_path);
                process::exit(1);
            }
            ErrorKind::IsADirectory => {
                log_error!("O caminho {} é um diretório!", args.file_path);
                process::exit(1);
            }
            _ => {
                log_error!("Não foi possível ler o arquivo!");
                process::exit(1);
            }
        },
    };

    log_info!("Gerando tokens...");
    let tokens_vec = ndr::tokenize(&ndr_code).unwrap_or(vec![]);
    println!("{}", ndr_code);
    println!("{:?}", tokens_vec);
    process::exit(0);
}

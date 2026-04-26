pub mod backend;
pub mod cli;
pub mod error;
pub mod frontend;
pub mod logging;

use std::path::Path;
use std::{fs, io::ErrorKind, process};

use backend::{assembler, assembly};
use clap::Parser;
use cli::Args;
use colored::*;
use frontend::{lexer, parser};

use crate::backend::emulator::NeanderEmulator;

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
    let tokens_vec = match lexer::tokenize(&ndr_code) {
        Ok(tokens) => tokens,
        Err(err) => {
            log_error!("{err}");
            process::exit(2);
        }
    };
    log_success!("Tokens gerados!");

    log_info!("Gerando AST...");
    let mut parser = parser::Parser::new(tokens_vec);
    let program = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            log_error!("{err}");
            process::exit(3);
        }
    };
    log_success!("AST criada!");

    log_info!("Construindo assembly...");
    let assembly_src = assembly::generate(&program);
    if args.save_asm {
        let path = Path::new(&args.file_path);

        if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
            let output_path = format!("{file_name}.asm");
            log_info!("Salvando assembly em {}", output_path);

            if let Err(err) = fs::write(&output_path, &assembly_src) {
                log_warn!("Erro ao salvar {}: {}", output_path, err);
            }
        } else {
            log_warn!("Não foi possível extrair o nome do arquivo para salvar o .asm");
        }
    }
    log_success!("Assembly criado!");

    let mem = match assembler::build(&assembly_src) {
        Ok(mem) => mem,
        Err(err) => {
            log_error!("{err}");
            process::exit(4);
        }
    };

    if args.save_mem {
        let path = Path::new(&args.file_path);
        if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
            let output_path = format!("{file_name}.mem");
            log_info!("Salvando bytecode em {}", output_path);
            mem.save_to_file(&output_path).unwrap();
        } else {
            log_warn!("Não foi possível extrair o nome do arquivo para salvar o .mem");
        }
    }

    log_info!("Rodando emulador...");
    let mut emulator = NeanderEmulator::new(mem);

    match emulator.run() {
        Err(err) => {
            log_error!("{err}");
            process::exit(5);
        }
        Ok(_) => {}
    };

    log_success!("Feito! Finalizando...");
    process::exit(0);
}

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

    log_info!("Gerando AST...");
    let mut parser = parser::Parser::new(tokens_vec.clone());
    let program = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            log_error!("{err}");
            process::exit(3);
        }
    };

    println!("Código:");
    println!("{}", ndr_code);
    println!("");
    println!("Tokens:");
    println!("{:#?}", tokens_vec);
    println!("");
    println!("Parser:");
    println!("{:#?}", program);

    log_info!("Construindo assembly...");
    let assembly_src = assembly::generate(&program);

    println!("Assembly:");
    println!("{}", assembly_src);

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

    process::exit(0);
}

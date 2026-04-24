pub mod cli;
pub mod frontend;
pub mod logging;

use std::{fs, io::ErrorKind, process};

use clap::Parser;
use cli::Args;
use colored::*;
use frontend::lexer;
use frontend::parser;

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
    process::exit(0);
}

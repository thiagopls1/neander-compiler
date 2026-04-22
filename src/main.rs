pub mod cli;
pub mod ndr;

use std::{fs, io::ErrorKind, process};

use clap::Parser;
use cli::Args;
use colored::*;

macro_rules! print_err {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "error:".red().bold(), format!($($arg)*));
    };
}

fn main() {
    let args = Args::parse();

    let ndr_code = match fs::read_to_string(&args.file_path) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                print_err!("Arquivo {} não encontrado!", args.file_path);
                process::exit(1);
            }
            ErrorKind::IsADirectory => {
                print_err!("O caminho {} é um diretório!", args.file_path);
                process::exit(1);
            }
            _ => {
                print_err!("Não foi possível ler o arquivo!");
                process::exit(1);
            }
        },
    };

    let tokens_vec = ndr::tokenize(&ndr_code).unwrap_or(vec![]);
    println!("{}", ndr_code);
    println!("{:?}", tokens_vec);
    process::exit(0);
}

pub mod cli_args;
pub mod tokens;

use std::{fs, io::ErrorKind, process};

use clap::Parser;
use cli_args::Args;

fn main() {
    let args = Args::parse();

    let ndr_code = match fs::read_to_string(&args.file_path) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                eprintln!("ERRO: Arquivo não encontrado!");
                process::exit(1);
            }
            ErrorKind::IsADirectory => {
                eprintln!("ERRO: O caminho passado é um diretório!");
                process::exit(1);
            }
            _ => {
                eprintln!("ERRO: Não foi possível ler o arquivo!");
                process::exit(1);
            }
        },
    };

    println!("{}", ndr_code);
    process::exit(0);
}

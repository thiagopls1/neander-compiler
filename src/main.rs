use std::{env, fs, io::ErrorKind, process};

fn main() {
    let src_code_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("ERRO: É necessário passar o caminho do arquivo de código-fonte!");
        process::exit(1);
    });

    let ndr_code = match fs::read_to_string(&src_code_path) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                eprintln!("ERRO: Arquivo não encontrado!");
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

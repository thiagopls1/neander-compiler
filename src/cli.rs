use clap::Parser;

///  A Neander compiler written in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File path of Neander code
    #[arg(short, long, value_name = "FILE")]
    pub file_path: String,
    /// Generates a Neander Assembly file
    #[arg(short = 'a', long)]
    pub save_asm: bool,

    /// Generates a Neander Bytecode (.mem) file
    #[arg(short = 'm', long)]
    pub save_mem: bool,
}

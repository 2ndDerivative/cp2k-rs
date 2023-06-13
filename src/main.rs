use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    option: Option<String>,
}

fn main() {
    let _args = Args::parse();
    let _input_file_name = PathBuf::new();
    let _output_file_name = PathBuf::new();
    let _command = std::env::current_exe().expect("should get exe name!");
}

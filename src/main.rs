use clap::Parser;
use std::{
    path::PathBuf,
    io::Write
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    input_file: PathBuf,

    #[arg(short)]
    output_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let mut output: Box<dyn Write> = args.output_file
        .map(|x| Box::new(std::fs::File::open(x).unwrap()))
        .unwrap_or_else(|| Box::new(std::io::stdout()));

    if let Some(output_file) = args.output_file {
        eprintln!("Output file chosen: {}", output_file.display());
    } else {
        eprintln!("Output to terminal");
    };
    write!(&mut output, "Input file: {}", args.input_file.display());

    let full_command = std::env::current_exe().expect("Should get exe name!");
    let command = full_command.file_name();
    write!(&mut output, "{command:?}");
}

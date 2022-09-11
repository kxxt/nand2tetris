mod command;
mod errors;
mod parser;
mod segment;
mod translation_state;
mod translator;

use clap::Parser as CmdlineParser;
use colored::*;
use std::{error::Error, fmt::Debug, fs, path::PathBuf};

use parser::Parser;
use translator::Translator;

#[derive(CmdlineParser, Debug)]
#[clap(author="kxxt", version, about="Jack VM code translator for nand2tetris course", long_about = None)]
struct Args {
    /// input file
    #[clap(value_parser)]
    file: String,

    /// output file
    #[clap(short, long, value_parser)]
    output: Option<String>,
}

fn handle_error(err: &Box<dyn Error>) {
    println!("{}: {}", "Error".bright_red(), err);
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(&args.file).expect("Error reading source file!");
    let commands = Parser::parse(&source)?;
    let asm = Translator::translate(commands.into_iter())?;
    let output_path: PathBuf = if let Some(output_path) = args.output {
        output_path.into()
    } else {
        let mut p = PathBuf::from(args.file);
        p.set_extension("asm");
        p
    };
    fs::write(output_path, asm)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(error) = run(args) {
        handle_error(&error);
    }
}

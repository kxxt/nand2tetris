mod command;
mod errors;
mod parser;
mod translation_state;
mod translator;
mod segment;

use clap::Parser as CmdlineParser;
use colored::*;
use std::{fmt::Debug, fs, path::PathBuf, process};

use errors::Error;
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

fn handle_error<T, E>(result: &Result<T, E>) -> bool
where
    E: Error,
    T: Debug,
{
    if result.is_err() {
        println!(
            "{}: {}",
            "Error".bright_red(),
            result.as_ref().unwrap_err().get_message()
        );
        true
    } else {
        false
    }
}

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(&args.file).expect("Error reading source file!");
    let result = Parser::parse(&source);
    if handle_error(&result) {
        process::exit(111);
    }
    let result = Translator::translate(result.unwrap().into_iter());
    if handle_error(&result) {
        process::exit(222);
    }
    let output_path: PathBuf = if let Some(output_path) = args.output {
        output_path.into()
    } else {
        let mut p = PathBuf::from(args.file);
        p.set_extension("asm");
        p
    };
    let result = fs::write(output_path, result.unwrap());
    if result.is_err() {
        println!("{}: {}", "Error".bright_red(), result.as_ref().unwrap_err());
        process::exit(333);
    }
}

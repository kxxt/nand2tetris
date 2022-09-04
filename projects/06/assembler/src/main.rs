mod assembler;
mod line_translator;

use clap::Parser;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author="kxxt", version, about="Hack assembler for nand2tetris course", long_about = None)]
struct Args {
    /// input file
    #[clap(value_parser)]
    file: String,

    /// output file
    #[clap(short, long, value_parser)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let lines = fs::read_to_string(args.file)
        .expect("Error reading source file!")
        .lines()
        .map(str::to_string)
        .collect();
    let mut assembler = assembler::Assembler::new(lines);
    let compiled = assembler.compile();
    fs::write(
        args.output.unwrap_or("a.out".to_string()),
        compiled.join("\n"),
    )
    .expect("Failed to write output to file!");
}

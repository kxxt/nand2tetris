#![feature(is_some_with)]
mod command;
mod errors;
mod parser;
mod segment;
mod source;
mod translation_state;
mod translator;

use clap::Parser as CmdlineParser;
use colored::*;
use std::{
    error::Error,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

use parser::Parser;
use source::Source;
use translator::Translator;

#[derive(CmdlineParser, Debug)]
#[clap(author="kxxt", version, about="Jack VM code translator for nand2tetris course", long_about = None)]
struct Args {
    /// input file
    #[clap(value_parser)]
    input: String,

    /// output file
    #[clap(short, long, value_parser)]
    output: Option<String>,
}

fn handle_error(err: &Box<dyn Error>) {
    println!("{}: {}", "Error".bright_red(), err);
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let input = Path::new(&args.input);
    let files = if input.is_dir() {
        fs::read_dir(input)?
            .filter_map(|s| {
                s.ok().and_then(|entry| {
                    let path = entry.path();
                    if path.is_file() && path.extension().is_some_and(|ext| *ext == "vm") {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
            .collect()
    } else {
        vec![PathBuf::from(input)]
    };
    if files.is_empty() {
        return Err(format!(
            "No source code found in directory {}!",
            input.to_string_lossy()
        )
        .into());
    }
    let sources = files.iter().map(|file| Source {
        content: fs::read_to_string(&file)
            .expect(&format!("Error reading {}!", file.to_string_lossy())),
        name: Path::new(file)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string(),
    });
    let parsed_sources = sources
        .map(|source| Parser::parse(source))
        .collect::<Result<Vec<_>, _>>()?;
    let asms = parsed_sources
        .iter()
        .map(|parsed| Translator::translate(parsed))
        .collect::<Result<Vec<_>, _>>()?;
    let output_path: PathBuf = if let Some(output_path) = args.output {
        output_path.into()
    } else {
        if input.is_file() {
            let mut p = PathBuf::from(input);
            p.set_extension("asm");
            p
        } else {
            input.join(Path::new(input.file_name().unwrap()).with_extension("asm"))
        }
    };
    let bootstrap = if input.is_dir() {
        Translator::BOOTSTRAP
    } else {
        ""
    };
    fs::write(output_path, format!("{bootstrap}{}", asms.join("\n")))?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(error) = run(args) {
        handle_error(&error);
    }
}

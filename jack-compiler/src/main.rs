#![feature(is_some_with)]
mod ast;
mod cli;
mod compiler;
mod emitter;
mod errors;
mod parser;
mod token;
mod tokenizer;

use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use cli::Args;
use compiler::Compiler;

fn run() -> Result<()> {
    let args = Args::parse();
    let inputs = args.get_inputs()?;
    let vmcodes = inputs
        .into_iter()
        .map(Compiler::compile)
        .collect::<Result<Vec<_>, anyhow::Error>>()?;
    let mut input = PathBuf::from(args.input);
    let input_is_single_file = input.is_file();
    let output_path_base: PathBuf = if let Some(output_path) = args.output {
        // --output provided
        output_path.into()
    } else {
        // --output not provided
        if input_is_single_file {
            input.set_extension("vm");
            input
        } else {
            input
        }
    };
    for (vmcode, name) in vmcodes {
        fs::write(
            if input_is_single_file {
                output_path_base.clone()
            } else {
                output_path_base.clone().join(name + ".vm")
            },
            vmcode,
        )?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {e}");
    }
}

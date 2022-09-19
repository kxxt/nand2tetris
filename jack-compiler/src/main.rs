#![feature(is_some_with)]
#![ allow( dead_code, unused_imports ) ]
mod ast;
mod cli;
mod parser;
mod token;
mod emitter;
mod tokenizer;
mod compiler;
mod errors;

fn main() {
    println!("Hello, world!");
}

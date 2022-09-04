mod assembler;
mod line_translator;

fn main() {
    let lines = vec![];
    let mut _assembler = assembler::Assembler::new(lines);
    _assembler.compile();
}

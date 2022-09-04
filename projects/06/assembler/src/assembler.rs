use crate::line_translator::LineTranslator;

pub struct Assembler {
    lines: Vec<String>,
    translator: LineTranslator,
}

impl Assembler {
    pub fn new(lines: Vec<String>) -> Assembler {
        Assembler {
            translator: LineTranslator::new(),
            // ASM Input
            lines,
        }
    }

    fn first_pass(&mut self) {
        self.lines = self
            .lines
            .iter()
            .filter_map(|line| self.translator.preprocess_line(line))
            .collect();
    }

    fn second_pass(&mut self) -> Vec<String> {
        self.lines
            .iter()
            .map(|line| self.translator.compile_line(line))
            .collect()
    }

    pub fn compile(&mut self) -> Vec<String> {
        self.first_pass();
        self.second_pass()
    }
}

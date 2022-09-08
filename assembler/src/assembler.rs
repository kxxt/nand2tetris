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

#[cfg(test)]
mod tests {
    use super::Assembler;

    #[test]
    fn it_works() {
        let lines: Vec<String> = r#"// Comment
(START)
@R2
D = A
@3
MD=D+A
@j
M=!M
(END)
@END
1;JMP"#
            .lines()
            .map(str::to_string)
            .collect();
        let compiled = Assembler::new(lines).compile();
        assert_eq!(
            compiled,
            r"0000000000000010
1110110000010000
0000000000000011
1110000010011000
0000000000010000
1111110001001000
0000000000000110
1110111111000111"
                .lines()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
    }
}

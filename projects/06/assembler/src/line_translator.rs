use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct LineTranslator {
    map: HashMap<String, u16>,
    reg_counter: u16,
    line_number: u16,
}

impl LineTranslator {
    pub fn new() -> LineTranslator {
        LineTranslator {
            map: HashMap::from(
                [
                    ("R0", 0u16),
                    ("R1", 1),
                    ("R2", 2),
                    ("R3", 3),
                    ("R4", 4),
                    ("R5", 5),
                    ("R6", 6),
                    ("R7", 7),
                    ("R8", 8),
                    ("R9", 9),
                    ("R10", 10),
                    ("R11", 11),
                    ("R12", 12),
                    ("R13", 13),
                    ("R14", 14),
                    ("R15", 15),
                    ("SP", 0),
                    ("LCL", 1),
                    ("ARG", 2),
                    ("THIS", 3),
                    ("THAT", 4),
                    ("SCREEN", 16384),
                    ("KBD", 24576),
                ]
                .map(|(symbol, addr)| (symbol.to_owned(), addr)),
            ),
            // Builtin reg: 0-15
            // User-defined: 16+
            reg_counter: 16,
            line_number: 0,
        }
    }

    pub fn preprocess_line(&mut self, line: &str) -> Option<String> {
        let line = line.trim();
        if line.len() == 0 || line.starts_with("//") {
            None
        } else if line.starts_with("(") {
            let label = line.trim_matches(|c| c == '(' || c == ')');
            self.map.insert(label.trim().to_string(), self.line_number);
            None
        } else {
            let mut line = line.to_string();
            // remove whitespace
            line.retain(|c| !char::is_whitespace(c));
            self.line_number += 1;
            Some(line)
        }
    }

    pub fn compile_line(&mut self, line: &str) -> String {
        if line.starts_with('@') {
            // A instruction
            self.compile_a_instruction(line)
        } else {
            self.compile_c_instruction(line)
        }
    }

    fn compile_a_instruction(&mut self, line: &str) -> String {
        let value = line.trim_start_matches('@');
        if let Ok(value) = value.parse::<u16>() {
            return Self::format_a_instruction(value);
        } else {
            if let Some(value) = self.map.get(value) {
                return Self::format_a_instruction(*value);
            } else {
                // found new variable
                self.map.insert(value.to_string(), self.reg_counter);
                let ret = Self::format_a_instruction(self.reg_counter);
                self.reg_counter += 1;
                return ret;
            }
        }
    }

    fn format_a_instruction(value: u16) -> String {
        // zero out highest bit
        let instruction = value & (u16::MAX >> 1);
        return format!("{instruction:016b}");
    }

    fn compile_c_instruction(&mut self, line: &str) -> String {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"((?P<dest>[ADM]+)=)?(?P<comp>[ADM+-01!&|]+)(;(?P<jmp>\w+))?").unwrap();
            static ref CMAP: HashMap<&'static str, u8> = HashMap::from([
                ("0", 0b0101010u8),
                ("1", 0b0111111),
                ("-1", 0b0111010),
                ("D", 0b0001100),
                ("A", 0b0110000),
                ("!D", 0b0001101),
                ("!A", 0b0110001),
                ("-D", 0b0001111),
                ("-A", 0b0110011),
                ("D+1", 0b0011111),
                ("A+1", 0b0110111),
                ("D-1", 0b0001110),
                ("A-1", 0b0110010),
                ("D+A", 0b0000010),
                ("D-A", 0b0010011),
                ("A-D", 0b0000111),
                ("D&A", 0b0000000),
                ("D|A", 0b0010101),
                ("M", 0b1110000),
                ("!M", 0b1110001),
                ("-M", 0b1110011),
                ("M+1", 0b1110111),
                ("M-1", 0b1110010),
                ("D+M", 0b1000010),
                ("D-M", 0b1010011),
                ("M-D", 0b1000111),
                ("D&M", 0b1000000),
                ("D|M", 0b1010101)
            ]);
        }
        let captures = RE
            .captures(line)
            .expect("No value got captured! Invalid C instruction!");
        let mut dest_bits = [false; 3]; // dest bits: ADM
        if let Some(dest) = captures.name("dest") {
            let dest = dest.as_str();
            dest_bits[0] = dest.contains('A');
            dest_bits[1] = dest.contains('D');
            dest_bits[2] = dest.contains('M');
        }
        let mut jump_bits = [false; 3];
        if let Some(jump) = captures.name("jmp") {
            let jump = jump.as_str();
            jump_bits = match jump {
                "JGT" => [false, false, true],
                "JEQ" => [false, true, false],
                "JGE" => [false, true, true],
                "JLT" => [true, false, false],
                "JNE" => [true, false, true],
                "JLE" => [true, true, false],
                "JMP" => [true, true, true],
                _ => panic!("Invalid jump command!"),
            }
        }
        let comp = captures
            .name("comp")
            .expect("Invalid comp part of C instruction!")
            .as_str();
        let comp_bin = *CMAP.get(comp).expect("Invalid computation operation!");
        let mut binary = (0b111u16 << 13) + ((comp_bin as u16) << 6);
        binary |= (dest_bits[0] as u16) << 5;
        binary |= (dest_bits[1] as u16) << 4;
        binary |= (dest_bits[2] as u16) << 3;
        binary |= (jump_bits[0] as u16) << 2;
        binary |= (jump_bits[1] as u16) << 1;
        binary |= jump_bits[2] as u16;
        return format!("{binary:016b}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_a_instruction_with_number(line: &str, compare: &str) {
        let mut translator = LineTranslator::new();
        let preprocessed = translator.preprocess_line(line).unwrap();
        assert_eq!(preprocessed, line);
        assert_eq!(translator.line_number, 1);
        let compiled = translator.compile_line(&preprocessed);
        assert_eq!(compiled, compare);
    }

    fn check_c_instruction(line: &str, compare: &str) {
        let mut translator = LineTranslator::new();
        let preprocessed = translator.preprocess_line(line).unwrap();
        assert_eq!(translator.line_number, 1);
        let compiled = translator.compile_line(&preprocessed);
        assert_eq!(compiled, compare);
    }
    #[test]
    fn a_instruction_number() {
        check_a_instruction_with_number("@14324", "0011011111110100");
        check_a_instruction_with_number("@25663", "0110010000111111");
    }
    #[test]
    fn c_instruction() {
        check_c_instruction("A = 0; JMP", "1110101010100111");
        check_c_instruction("AMD = 0; JMP", "1110101010111111");
        check_c_instruction("1;JGT", "1110111111000001");
        check_c_instruction("   D= D & M", "1111000000010000");
    }

    #[test]
    fn label() {
        let mut translator = LineTranslator::new();
        let preprocessed = translator.preprocess_line("   (  LABEL    )  ");
        assert_eq!(preprocessed, None);
        assert_eq!(translator.line_number, 0);
        let compiled = translator.compile_line("@LABEL");
        assert_eq!(compiled, "0000000000000000");
        translator.preprocess_line("0");
        translator.compile_line("0");
        let preprocessed = translator.preprocess_line("   (  L    )  ");
        assert_eq!(preprocessed, None);
        assert_eq!(translator.line_number, 1);
        let compiled = translator.compile_line("@L");
        assert_eq!(compiled, "0000000000000001");
    }

    #[test]
    fn empty() {
        let mut translator = LineTranslator::new();
        let preprocessed = translator.preprocess_line("  // Wow!  ");
        assert_eq!(preprocessed, None);
        assert_eq!(translator.line_number, 0);
        let preprocessed = translator.preprocess_line("  ");
        assert_eq!(preprocessed, None);
        assert_eq!(translator.line_number, 0);
    }

    #[test]
    #[should_panic]
    fn invalid() {
        let mut translator = LineTranslator::new();
        let line = "#asdfjk*()O";
        let line = translator.preprocess_line(line);
        let _compiled = translator.compile_line(&line.unwrap());
    }
}

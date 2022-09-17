use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

use super::token::{Token, TokenKind};

pub struct Source {
    pub content: String,
    pub name: String,
}

pub struct Tokenizer<'a> {
    source: String,
    module_name: String,
    iterator: Cow<'a, str>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: Source) -> Self {
        lazy_static! {
            static ref REMOVE_COMMENTS: Regex = Regex::new(r#"\/\*(.|\n)*?\*\/|\/\/.*"#).unwrap();
        }
        let Source {
            content: source,
            name: module_name,
        } = source;
        Tokenizer {
            source,
            module_name,
            iterator: REMOVE_COMMENTS.replace_all(&source, ""),
        }
    }

    fn source_name(&self) -> &str {
        return &self.module_name;
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_comments() {
        let source = Source {
            name: "Test".to_string(),
            content: r##"
/** implements particle
system simulation.*/
class ParticleSystem {
    field int count;
    field Array particles;
    constructor ParticleSystem new() {
        let count = 0;
        return this;
    }

    /** Update the simulation by one step. */
    method void update() {
        var int i;
        var Particle cur;
        let i = 0;
        while (i < count) {
            let cur = particles[i]; 
            do cur.update();
            let i = i + 1;
        }
        return;
    }

    /** Randomly generate all particles. */
    method void generate() {
        var int i, xsign, ysign;
        var Particle p;
        let i = 0;
        let xsign = 1;
        let ysign = 1;
        while (i < count) {
            if (Random.rand() > 16383) {
                let xsign = -xsign;
            }
            if (Random.rand() > 16383) {
                let ysign = -ysign;
            }
            let p = Particle.new(Random.randRange(508-3)+3, Random.randRange(252-3)+3, 
                        xsign * Random.randRange(3), ysign * Random.randRange(3));
            let particles[i] = p;
            let i = i + 1;
        }
        return;
    }

    /** Runs the simulation: quit by pressing q */
    method void run() {
        var char key;  // the key currently pressed by the user
        var boolean exit;
        do Random.setSeed(2333);
        do Output.printString("Welcome to particle system!");
        do Output.println();
        let count = Keyboard.readInt("How many particles do you want to render?> ");
        while ((count < 0) | (count = 0)) {
            let count = Keyboard.readInt("Please enter a positive number!> ");
        }
        do Output.printString("Generating particles, please wait...");
        do Output.println();
        let particles = Array.new(count);
        do generate();
        do Output.printString("Done generation! Press any key to start simulation.");
        do Output.println();
        do Keyboard.readChar();
        do Screen.clearScreen();
        let exit = false;
        
        while (~exit) {
            let key = Keyboard.keyPressed();
            do update();
            if (key = 81)  { let exit = true; }     // q key
        }
        return;
    }

    method void dispose() {
        do particles.dispose();
        do Memory.deAlloc(this);
        return;
    }
}"##
            .to_string(),
        };
        let tokenizer = Tokenizer::new(source);
        assert_eq!(tokenizer.iterator.to_string(), r##"

class ParticleSystem {
    field int count;
    field Array particles;
    constructor ParticleSystem new() {
        let count = 0;
        return this;
    }

    
    method void update() {
        var int i;
        var Particle cur;
        let i = 0;
        while (i < count) {
            let cur = particles[i]; 
            do cur.update();
            let i = i + 1;
        }
        return;
    }

    
    method void generate() {
        var int i, xsign, ysign;
        var Particle p;
        let i = 0;
        let xsign = 1;
        let ysign = 1;
        while (i < count) {
            if (Random.rand() > 16383) {
                let xsign = -xsign;
            }
            if (Random.rand() > 16383) {
                let ysign = -ysign;
            }
            let p = Particle.new(Random.randRange(508-3)+3, Random.randRange(252-3)+3, 
                        xsign * Random.randRange(3), ysign * Random.randRange(3));
            let particles[i] = p;
            let i = i + 1;
        }
        return;
    }

    
    method void run() {
        var char key;  
        var boolean exit;
        do Random.setSeed(2333);
        do Output.printString("Welcome to particle system!");
        do Output.println();
        let count = Keyboard.readInt("How many particles do you want to render?> ");
        while ((count < 0) | (count = 0)) {
            let count = Keyboard.readInt("Please enter a positive number!> ");
        }
        do Output.printString("Generating particles, please wait...");
        do Output.println();
        let particles = Array.new(count);
        do generate();
        do Output.printString("Done generation! Press any key to start simulation.");
        do Output.println();
        do Keyboard.readChar();
        do Screen.clearScreen();
        let exit = false;
        
        while (~exit) {
            let key = Keyboard.keyPressed();
            do update();
            if (key = 81)  { let exit = true; }     
        }
        return;
    }

    method void dispose() {
        do particles.dispose();
        do Memory.deAlloc(this);
        return;
    }
}"##);
    }
}

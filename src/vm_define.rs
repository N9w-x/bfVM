use std::io::{Read, Stdin, stdin, Stdout, stdout, Write};

use crate::parser::{AstNode, Parse};
use crate::SimpleParser;

type Parser = Box<dyn Parse>;

pub struct VM<const RAM_SIZE: usize> {
    mem: Box<[u8; RAM_SIZE]>,
    rom: Vec<AstNode>,
    pointer: usize,
    #[allow(dead_code)]
    parser: Parser,
    std_in: Stdin,
    std_out: Stdout,
}

impl<const RAM_SIZE: usize> Default for VM<RAM_SIZE> {
    fn default() -> Self {
        Self {
            mem: Box::new([0; RAM_SIZE]),
            rom: vec![],
            pointer: 0,
            parser: Box::new(SimpleParser),
            std_in: stdin(),
            std_out: stdout(),
        }
    }
}

// inner functions
impl<const RAM_SIZE: usize> VM<RAM_SIZE> {
    fn get_input(&mut self) {
        let mut buf = [0u8; 1];
        let len = match self.std_in.read(&mut buf) {
            Ok(x) => x,
            Err(_) => { panic!("read fail") }
        };
        assert_eq!(len, 1);
        self.mem[self.pointer] = buf[0];
    }
    
    fn get_output(&mut self) {
        let buf = [self.mem[self.pointer]];
        if let Ok(x) = self.std_out.write(&buf) { assert_eq!(x, 1); };
    }
    
    // calc add and sub
    // when sub,param should changed to it's 2'complement
    fn calc(&mut self, oth: u8) {
        let (val, _) = self.mem[self.pointer].overflowing_add(oth);
        self.mem[self.pointer] = val;
    }
    
    fn pointer_move(&mut self, oth: usize) {
        if oth == usize::MAX {
            if self.pointer == 0 {
                self.pointer = RAM_SIZE - 1;
            } else {
                let (val, _) = self.pointer.overflowing_add(oth);
                self.pointer = val;
            }
        } else {
            self.pointer = (self.pointer + oth) % RAM_SIZE;
        }
    }
    
    fn get_unit(&self) -> u8 {
        self.mem[self.pointer]
    }
    
    fn execute(&mut self, node_list: Vec<AstNode>) {
        for node in node_list {
            match node {
                AstNode::PointerInc { val } => self.pointer_move(val),
                AstNode::Inc { val } => self.calc(val),
                AstNode::Read => self.get_input(),
                AstNode::Write => self.get_output(),
                AstNode::LoopBlock {
                    block, ..
                } => while self.get_unit() != 0 {
                    self.execute(block.clone());
                    if self.get_unit() == 0 {
                        break;
                    };
                },
            }
        }
    }
}

impl<const RAM_SIZE: usize> VM<RAM_SIZE> {
    pub fn new(path: &str, parser: Parser) -> Self {
        let rom = parser.parse(path);
        Self {
            mem: Box::new([0; RAM_SIZE]),
            rom,
            pointer: 0,
            parser,
            std_in: stdin(),
            std_out: stdout(),
        }
    }
    
    pub fn exec(&mut self) {
        self.execute(self.rom.clone());
    }
    
    pub fn read_from_stdin(&mut self) {
        let mut source = String::new();
        self.std_in.read_line(&mut source).unwrap();
        let vec = self.parser.parse_from_stdin(source);
        self.execute(vec);
    }
}


use std::io::{Read, Stdin, stdin, Stdout, stdout, Write};

use crate::parser::{AstNode, OpCode};
use crate::parser::AstNode::LoopBlock;

pub struct VM<const T: usize, const ROM_SIZE: usize> {
    mem: Box<[u8; T]>,
    pointer: usize,
    jump_addr: (usize, usize),
    jump_stack: Vec<(usize, usize)>,
    std_in: Stdin,
    std_out: Stdout,
}

impl<const T: usize, const ROM_SIZE: usize> VM<T, ROM_SIZE> {
    //pub fn new(_rom: Vec<OpCode>, jump_addr_stack: Vec<(usize, usize)>) -> Self {
    //    Self {
    //        mem: Box::new([0; T]),
    //        pointer: 0,
    //        jump_addr: (0, 0),
    //        jump_stack: jump_addr_stack,
    //        std_in: stdin(),
    //        std_out: stdout(),
    //    }
    //}
    
    pub fn new() -> Self {
        Self {
            mem: Box::new([0; T]),
            pointer: 0,
            jump_addr: (0, 0),
            jump_stack: vec![],
            std_in: stdin(),
            std_out: stdout(),
        }
    }
}


impl<const T: usize, const ROM_SIZE: usize> VM<T, ROM_SIZE> {
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
                self.pointer = T - 1;
            } else {
                let (val, _) = self.pointer.overflowing_add(oth);
                self.pointer = val;
            }
        } else {
            self.pointer = (self.pointer + oth) % T;
        }
    }
    
    fn get_unit(&self) -> &u8 {
        &self.mem[self.pointer]
    }
    
    pub fn execute(&mut self, node_list: Vec<AstNode>) {
        for node in node_list {
            match node {
                AstNode::PointerInc { val } => self.pointer_move(val),
                AstNode::Inc { val } => self.calc(val),
                AstNode::Read => self.get_input(),
                AstNode::Write => self.get_output(),
                AstNode::LoopBlock {
                    block, ..
                } => while self.mem[self.pointer] != 0 {
                    self.execute(block.clone());
                    if self.mem[self.pointer] == 0 {
                        break;
                    };
                },
            }
        }
    }
}


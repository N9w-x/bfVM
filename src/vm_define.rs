use std::io::{Read, Stdin, stdin, Stdout, stdout, Write};

use crate::parser::OpCode;

pub struct VM<const T: usize, const ROM_SIZE: usize> {
    mem: Box<[u8; T]>,
    rom: Box<[OpCode; ROM_SIZE]>,
    pointer: usize,
    jump_addr: (usize, usize),
    jump_stack: Vec<(usize, usize)>,
    std_in: Stdin,
    std_out: Stdout,
}

impl<const T: usize, const ROM_SIZE: usize> VM<T, ROM_SIZE> {
    pub fn new(_rom: Vec<OpCode>, jump_addr_stack: Vec<(usize, usize)>) -> Self {
        Self {
            mem: Box::new([0; T]),
            rom: Box::new([OpCode::Null; ROM_SIZE]),
            pointer: 0,
            jump_addr: (0, 0),
            jump_stack: jump_addr_stack,
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
    
    fn get_unit(&self) -> &u8 {
        &self.mem[self.pointer]
    }
    
    pub fn execute(&mut self, op_vec: Vec<OpCode>) {
        assert!(op_vec.len() <= self.rom.len());
        // load
        for (pointer, op) in op_vec.into_iter().enumerate() {
            self.rom[pointer] = op;
        }
        
        //execute
        let mut pc = 0usize;
        while pc < self.rom.len() {
            match self.rom[pc] {
                OpCode::Add => self.pointer = (self.pointer + 1) % T,
                OpCode::Sub => {
                    let (t, over) = self.pointer.overflowing_sub(1);
                    if over {
                        self.pointer = T - 1;
                    } else {
                        self.pointer = t;
                    }
                }
                OpCode::Inc => self.calc(1),
                OpCode::Dec => self.calc(u8::MAX),
                OpCode::In => self.get_input(),
                OpCode::Out => self.get_output(),
                OpCode::JumpAfter(addr) => {
                    // check jump addr
                    if self.jump_addr.0 != addr {
                        self.jump_addr = self.jump_stack.pop().unwrap();
                    }
                    
                    if *self.get_unit() == 0 {
                        pc = self.jump_addr.1 + 1;
                    } else {
                        pc += 1;
                    }
                    continue;
                }
                OpCode::JumpBefore(_addr) => {
                    if *self.get_unit() != 0 {
                        pc = self.jump_addr.0 + 1;
                    } else {
                        pc += 1;
                    }
                    continue;
                }
                OpCode::Null => break
            }
            pc += 1;
        }
    }
}


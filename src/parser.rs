
use std::fs::File;
use std::io::Read;

use std::str::{self};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OpCode {
    Add,
    Sub,
    Inc,
    Dec,
    In,
    Out,
    JumpAfter(usize),
    JumpBefore(usize),
    Null,
}

pub struct Parser;

impl Parser {
    //read from file and return opcode and pair address
    pub fn reade(path: &str) -> (Vec<OpCode>, Vec<(usize, usize)>) {
        let mut file = File::open(path).expect("open file fail");
        let mut byte_vec: Vec<u8> = Vec::new();
        file.read_to_end(&mut byte_vec).unwrap();
        
        let mut op_vec: Vec<OpCode> = Vec::new();
        //jump after and jump before addr vec
        let mut addr_pair: Vec<(usize, usize)> = Vec::new();
        //temp store left bracket addr
        let mut left_bracket_addr = 0usize;
        
        //parse bf source code
        //todo add syntax checker
        for (addr, byte) in byte_vec.iter().enumerate() {
            match str::from_utf8(&[*byte]).unwrap() {
                ">" => op_vec.push(OpCode::Add),
                "<" => op_vec.push(OpCode::Sub),
                "+" => op_vec.push(OpCode::Inc),
                "-" => op_vec.push(OpCode::Dec),
                "." => op_vec.push(OpCode::Out),
                "," => op_vec.push(OpCode::In),
                "[" => {
                    op_vec.push(OpCode::JumpAfter(addr));
                    left_bracket_addr = addr;
                }
                "]" => {
                    op_vec.push(OpCode::JumpBefore(addr));
                    addr_pair.push((left_bracket_addr, addr));
                }
                _ => {}
            }
        };
        
        (op_vec, addr_pair)
    }
}
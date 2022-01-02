use std::fs::File;
use std::io::Read;
use std::str::{self};
use std::usize;

use crate::parser::AstNode::LoopBlock;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AstNode {
    PointerInc {
        val: usize
    },
    Inc {
        val: u8
    },
    Read,
    Write,
    LoopBlock {
        range: (usize, usize),
        block: Vec<AstNode>,
    },
}

pub struct Parser;

impl Parser {
    pub fn parse(path: &str) -> Vec<AstNode> {
        // read source code file to string
        let mut source = String::new();
        File::open(path).expect("open file fail").read_to_string(&mut source).unwrap();
        
        let mut inst = vec![];
        let mut stack = vec![];
        for (addr, op_code) in source.chars().enumerate() {
            match op_code {
                '+' => inst.push(AstNode::Inc { val: 1 }),
                '-' => inst.push(AstNode::Inc { val: u8::MAX }),
                '>' => inst.push(AstNode::PointerInc { val: 1 }),
                '<' => inst.push(AstNode::PointerInc { val: usize::MAX }),
                ',' => inst.push(AstNode::Read),
                '.' => inst.push(AstNode::Write),
                '[' => {
                    stack.push((inst, addr));
                    inst = vec![];
                }
                ']' => {
                    if let Some((mut block, start_addr)) = stack.pop() {
                        block.push(LoopBlock {
                            range: (start_addr, addr),
                            block: inst,
                        });
                        inst = block
                    }
                }
                _ => (),
            }
        };
        inst
    }
    
    //read from file and return opcode and pair address
    #[allow(dead_code)]
    pub fn read(path: &str) -> (Vec<OpCode>, Vec<(usize, usize)>) {
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
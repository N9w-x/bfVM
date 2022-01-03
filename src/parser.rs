use std::fs::File;
use std::io::Read;
use std::str::{self};
use std::usize;

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

pub trait Parse {
    fn parse(&self, path: &str) -> Vec<AstNode>;
    fn parse_from_stdin(&self, source: String) -> Vec<AstNode>;
}

pub struct SimpleParser;

impl Parse for SimpleParser {
    fn parse(&self, path: &str) -> Vec<AstNode> {
        // read source code file to string
        let mut source = String::new();
        File::open(path).expect("open file fail").read_to_string(&mut source).unwrap();
        self.parse_source(source)
    }
    
    fn parse_from_stdin(&self, source: String) -> Vec<AstNode> {
        self.parse_source(source)
    }
}

impl SimpleParser {
    fn parse_source(&self, source: String) -> Vec<AstNode> {
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
                        block.push(AstNode::LoopBlock {
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
}
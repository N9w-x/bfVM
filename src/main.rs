#![feature(bool_to_option)]

use crate::parser::Parser;
use crate::vm_define::VM;

mod parser;

mod vm_define;

#[deny(arithmetic_overflow)]
fn main() {
    let vec = Parser::parse("./test.bf");
    //for op in vec {
    //    println!("{:#?}", op);
    //}
    //let (op_vec, jump_addr_vec) = Parser::read("./test.bf");
    let mut bf_vm = VM::<4096, 4096>::new();
    bf_vm.execute(vec);
}

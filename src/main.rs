#![feature(bool_to_option)]

use crate::parser::Parser;
use crate::vm_define::VM;

mod parser;

mod vm_define;

#[deny(arithmetic_overflow)]
fn main() {
    let (op_vec, jump_addr_vec) = Parser::reade("./test.bf");
    let mut bf_vm = VM::<1024, 1024>::new(op_vec.clone(), jump_addr_vec);
    bf_vm.execute(op_vec);
}

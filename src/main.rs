#![allow(non_snake_case)]

use clap::clap_app;

use crate::parser::SimpleParser;
use crate::vm_define::VM;

mod parser;

mod vm_define;


// todo try to add simple optimize
fn main() {
    let args = clap_app!(BFVM =>
        (version: "0.0.1")
        (author: "wnx")
        (@arg Mem:-m -mem "选择虚拟机内存大小")
        (@arg File:-f --file  + takes_value "选择执行的文件")
    ).get_matches();
    
    let file = args.value_of("File");
    if file.is_some() {
        let mut vm = VM::<{ 1024 * 4 }>::new(file.unwrap(), Box::new(SimpleParser));
        vm.exec();
    } else {
        let mut vm = VM::<{ 1024 * 4 }>::default();
        vm.read_from_stdin();
    }
}

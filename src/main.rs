#![feature(str_split_as_str)]
#![allow(non_snake_case)]
#![feature(type_ascription)]

use std::fs::{File, read_dir};

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

#[test]
pub fn test() {
    //dir of testcase
    let dir = read_dir("./testcase").unwrap();
    let mut vm = VM::<4096>::default();
    for entry in dir {
        //get dirent
        let dirent = entry.unwrap();
        let filename = dirent.file_name().into_string().unwrap();
        
        println!("{:?}", filename.split('.').collect(): Vec<_>);
    }
}

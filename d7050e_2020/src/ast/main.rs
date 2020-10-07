use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

use crate::type_checker::*;
pub mod type_checker;
pub mod ast;

fn main() {/*
    let mut scopes = VecDeque::new();
    let mut var_env:HashMap<String, Type> = HashMap::new();
    let mut func_info:HashMap<String, Vec<String>> = HashMap::new();*/
    //println!("{:?}", ProgramParser::new().parse("let d:i32=5;d=4;"));
    let s = "fn a(a:i32) -> () {let mut b:i32=5;a(b)};";
    /* let s = "fn a(a:i32) -> () {let b:i32=5;a(b)};"; */
    println!("{:?}", type_checker(ProgramParser::new().parse(s).unwrap()));
}
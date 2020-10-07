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
    //println!("{:?}", ProgramParser::new().parse("fn a() -> i32 {b(55)+a()};fn b(c:i32) -> i32 {c = 3;c};"));
    println!("{:?}", type_checker(ProgramParser::new().parse("fn a(c:i32) -> i32 {b(c,true)+a(c)};fn b(mut c: i32,mut b:bool) -> i32 {c = 3;b =false;c};").unwrap()));
}
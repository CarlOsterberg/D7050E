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
    //println!("{:?}", ExprParser::new().parse("56-1*(5+3)").unwrap());
    println!("{:?}", type_checker(ProgramParser::new().parse("fn a() -> i32 {a()};fn b() -> i32 {5};").unwrap()));
}
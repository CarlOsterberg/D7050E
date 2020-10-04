use lalrpop_util::lalrpop_mod;

use std::collections::VecDeque;
use std::collections::HashMap;
use crate::ast::*;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

use crate::type_checker::*;
pub mod type_checker;
pub mod ast;

fn main() {
    let mut scopes = VecDeque::new();
    let mut func_info:HashMap<String, Vec<String>> = HashMap::new();
    let map:HashMap<String,Type> = HashMap::new();
    scopes.push_front(map);
    //println!("{:?}", &mut 55);
    //println!("{:?}", ExprParser::new().parse("let mut a:&i32 = 5"));
    println!("{:?}", type_checker(ProgramParser::new().parse("fn b(a:&i32) -> &i32 {a};").unwrap()));
}
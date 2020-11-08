use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/parser.rs");

use parser::*;

use crate::interpreter::*;
use crate::type_checker::*;
//use crate::ast::*;
pub mod type_checker;
pub mod interpreter;
pub mod ast;
use std::{fs,env};

fn main() {
    let mut dir = env::current_dir().unwrap();
    dir.push("src");
    dir.push("tests");
    dir.push("basic");
    dir.push("funcCall.rs");
    let s =fs::read_to_string(dir).unwrap();
    let parsed = ProgramParser::new().parse(&s);
    println!("Parser result: {:?}", &parsed);
    println!("Type checker result: {:?}", type_checker(parsed.clone().unwrap()));
    println!("Interpreter result: {:?}", interpreter(parsed.unwrap()));
}
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

use crate::type_checker::*;
pub mod type_checker;
pub mod ast;

fn main() {
    /* let mut scopes = VecDeque::new();
    let mut func_info:HashMap<String, Vec<String>> = HashMap::new();
    let map:HashMap<String,Type> = HashMap::new();
    scopes.push_front(map);
    println!("{:?}", &mut 55); */
    /*  let mut a = &5;
    let mut b = &mut a;
    let mut c = &mut b;
    *b = 6; */ 
    //println!("{:?}", type_checker(ProgramParser::new().parse("fn a() -> () {let mut a:i32 = 5;let b:&mut i32 = &mut a;*b=7;};").unwrap()));
    println!("{:?}", type_checker(ProgramParser::new().parse("fn a() -> i32 {let a:i32 = 5;a};").unwrap()));
    //println!("{:?}", type_checker(ProgramParser::new().parse("fn a() -> () {let a:i32 = 5;let b:&mut i32 = &mut a;*b=7;};").unwrap()));
}
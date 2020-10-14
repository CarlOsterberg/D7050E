use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/parser.rs");

use parser::*;

use crate::interpreter::*;
use crate::type_checker::*;
pub mod type_checker;
pub mod interpreter;
pub mod ast;

fn main() {
    //println!("{:?}", ProgramParser::new().parse("fn a() -> &i32 {a()};"));
    //let s = "fn a() -> & &i32 {& &b()};fn b() -> i32 {5};";
    //works
    //let s = "fn a() -> () {let mut a:i32 = 5;let b:&mut i32 = &mut a;*b=7;};";
    //crashes
    //let s = "fn a() -> () {let mut a:i32 = 5;let b:&i32 = &a;*b=7;};";
    //crashes
    //let s = "fn a() -> () {let a:i32 = 5;let b:&mut i32 = &mut a;*b=7;};";
    //let s = "fn a() -> bool {let a:bool = if true {true};else{false};a};";
    let s = "let mut a:i32 = 5";
    let p = ExprParser::new().parse(s.clone());
    interpreter(p.unwrap());

    /* let type_res = type_checker(p.unwrap());
    let mut check = true;
    for t in type_res {
        if t.is_err() {
            check = false;
            println!("type checker returned err");
        }
    }
    if check {
        
    } */
    
}
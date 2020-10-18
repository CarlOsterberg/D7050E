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
    
    /* let mut a = 5;
    let b = &a;
    let a = false;
    println!("{:?}",b); */

    let s = "fn main() -> i32 {let mut a:i32 = 5;let b:&mut i32 = &mut a;a(b);a}; fn a(c:&mut i32) -> () {if true {*c=1;};};";
    let p = ProgramParser::new().parse(s.clone());

    let type_res = type_checker(p.clone().unwrap());
    let mut check = true;
    for t in type_res {
        if t.is_err() {
            check = false;
            println!("type checking failed");
        }
    }
    if check {
        interpreter(p.clone().unwrap());
    }
    
}
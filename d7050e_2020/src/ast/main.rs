use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

use crate::type_checker::*;
pub mod type_checker;
pub mod ast;

fn main() {
    //println!("{:?}", ProgramParser::new().parse("b=& &7;"));
    let s = "fn a() -> & &i32 {let a:i32=5;let b:&i32=&a;let c:& &i32=&b;c};";
    //works
    //let s = "fn a() -> () {let mut a:i32 = 5;let b:&mut i32 = &mut a;*b=7;};";
    //crashes
    //let s = "fn a() -> () {let mut a:i32 = 5;let b:&i32 = &a;*b=7;};";
    //crashes
    //let s = "fn a() -> () {let a:i32 = 5;let b:&mut i32 = &mut a;*b=7;};";
    println!("{:?}", type_checker(ProgramParser::new().parse(s).unwrap()));
}
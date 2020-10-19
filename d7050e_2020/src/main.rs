use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/parser.rs");

use parser::*;

use crate::interpreter::*;
use crate::type_checker::*;
pub mod type_checker;
pub mod interpreter;
pub mod ast;
use std::{fs,env};


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
    let folder = "C:\\D7050E\\d7050e_2020\\src\\tests";
    let paths =fs::read_dir(folder);
    println!("{:?}",paths);
    for file in paths {

    }

    let path = "C:\\D7050E\\d7050e_2020\\src\\tests\\deref_funccall.rs";
    let s = fs::read_to_string(path).unwrap();
    let p = ProgramParser::new().parse(&s);

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

#[test]
fn parse_expr() {
    assert!(ProgramParser::new().parse("fn a() -> () {a=a();};").is_ok());
    assert!(ProgramParser::new().parse("fn a() -> i32 {a=a+2;};").is_ok());
    assert!(ProgramParser::new().parse("fn a(x:i32,y:bool) -> i32 {while (x<3) {x=x+1;y=false;}; if true {b=b;}; else {a=a;};a=b;};").is_ok());
    assert!(ProgramParser::new().parse("let a:i32 = if true {b=b;};else {a=a;};").is_ok());
    assert!(ProgramParser::new().parse("a=if true {b=b;}; else {a=a;};").is_ok());
    assert!(ProgramParser::new().parse("if true {b=b;};else {a=a;};").is_ok());
    assert!(ProgramParser::new().parse("let mut a:i32 = 1;").is_ok());
    assert!(ProgramParser::new().parse("let a:bool = true;").is_ok());
    assert!(ProgramParser::new().parse("while (x==5) {let x:i32 = 0;let x:i32 = 0;if true {b=b;};else {a=a;};};").is_ok());
    assert!(ProgramParser::new().parse("a=5+5*3;").is_ok());
}
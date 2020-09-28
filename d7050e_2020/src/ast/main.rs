use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::collections::VecDeque;


lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

use crate::type_checker::*;
use crate::ast::*;
pub mod type_checker;
pub mod ast;

fn main() {
    let mut scopes = VecDeque::new();
    let mut var_env:HashMap<String, Type> = HashMap::new();
    let func_info:HashMap<String, Type> = HashMap::new();
    scopes.push_front(var_env);
    println!("{:?}", ProgramParser::new().parse("fn a() -> () {a=a();a};").unwrap());
    //println!("{:?}", expr_type(ExprParser::new().parse("fn a(c:bool) -> bool {c}").unwrap(),&mut scopes,&func_info));
}

#[test]
fn parse_stmnt() {
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
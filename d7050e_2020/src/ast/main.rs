use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;


fn main() {
    //let _true:i32 = 22;
    println!("{:?}", ExprParser::new().parse("while true {let a:i32 = 5;let b:i32 = 5; a}"));
    //println!("{}", ExprParser::new().parse("-6+1*88").unwrap());
}

#[test]
fn parse_stmnt() {
    assert!(ExprParser::new().parse("fn a() -> () {a=a();}").is_ok());
}
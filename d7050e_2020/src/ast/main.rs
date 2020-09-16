use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;

fn main() {
    println!("{:?}", StmntParser::new().parse("fn a() -> () {
    a+2}"));
    //println!("{}", ExprParser::new().parse("-6+1*88").unwrap());
}

#[test]
fn parse_stmnt() {
    assert!(StmntParser::new().parse("fn a() -> () {a=a();}").is_ok());
    assert!(StmntParser::new().parse("fn a() -> () {a=a+2;}").is_ok());
    assert!(StmntParser::new().parse("fn a(x:i32,y:bool) -> i32 {while (x<3) {x=x+1;y=false;}if true {b=b;}else {a=a;}a=b;}").is_ok());
    assert!(StmntParser::new().parse("let a:i32 = if true {b=b;}else {a=a;};").is_ok());
    assert!(StmntParser::new().parse("a=if true {b=b;}else {a=a;};").is_ok());
    assert!(StmntParser::new().parse("if true {b=b;}else {a=a;}").is_ok());
    assert!(StmntParser::new().parse("let mut a:i32 = 1;").is_ok());
    assert!(StmntParser::new().parse("let a:bool = true;").is_ok());
    assert!(StmntParser::new().parse("while (x==5) {let x:i32 = 0;let x:i32 = 0;if true {b=b;}else {a=a;}}").is_ok());
    assert!(StmntParser::new().parse("a=5+5*3;").is_ok());
}

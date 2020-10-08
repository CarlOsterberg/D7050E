#[path ="../ast/ast.rs"]
pub mod ast;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser, "/ast/parser.rs");
use parser::*;
use std::fs;
use std::path::Path;

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
#[test]
fn r() {
    let path = Path::new("/type_checker_tests/UnaryTest.rs");
    let s = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
        assert!(ProgramParser::new().parse(&s).is_ok());
}
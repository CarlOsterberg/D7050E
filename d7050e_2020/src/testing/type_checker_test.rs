#[path ="../ast/ast.rs"]
pub mod ast;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser, "/ast/parser.rs");
use parser::*;
use std::env;
use std::fs;

#[test]
fn r() {
    let s = fs::read_to_string(UnaryTest.rs)
        .expect("Something went wrong reading the file");
        ProgramParser::new().parse(s);
        assert!(ProgramParser::new().parse(s).is_ok());
}
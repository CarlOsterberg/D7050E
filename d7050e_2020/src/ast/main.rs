use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;

fn main() {
    println!("{:?}", NumOrIdParser::new().parse("1+1-1"));
}

#[test]
fn parse_num_or_id() {
    assert_eq!(
        format!("{}", NumOrIdParser::new().parse("123").unwrap()),
        "123"
    );
    assert_eq!(
        format!("{}", NumOrIdParser::new().parse("a1_a").unwrap()),
        "a1_a"
    );
}

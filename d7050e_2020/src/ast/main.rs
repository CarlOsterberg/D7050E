use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;


fn main() {
    //let _true:i32 = 22;
    println!("{:?}", ExprParser::new().parse("while 89>x {while x>0{a};while x>0{a}}"));
    //println!("{}", ExprParser::new().parse("-6+1*88").unwrap());
}

#[test]
fn parse_expr() {
    assert_eq!(
        format!("{:?}", ExprParser::new().parse("2*1").unwrap()),
        "Op(Number(1), Greater, Op(Op(Number(1), Mul, Number(1)), Add, Number(2)))"
    );
}

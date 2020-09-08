use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;

fn main() {
    let b_32 = 1;
    println!("{:?}", ExprParser::new().parse("1*1+2 == 3").unwrap());
}

#[test]
fn parse_expr() {
    assert_eq!(
        format!("{:?}", ExprParser::new().parse("1>1*1+2").unwrap()),
        "Op(Number(1), Greater, Op(Op(Number(1), Mul, Number(1)), Add, Number(2)))"
    );
}

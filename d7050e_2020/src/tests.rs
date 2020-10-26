use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/parser.rs");

use parser::*;

use crate::interpreter::*;
use crate::type_checker::*;
//use crate::ast::*;
pub mod type_checker;
pub mod interpreter;
pub mod ast;
use std::{fs,env};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut dir = env::current_dir().unwrap();
        dir.push("src");
        dir.push("tests");
        dir.push("basic");
        let paths =fs::read_dir(dir);
        for file in paths.unwrap() {
            let s = fs::read_to_string(file.unwrap().path()).unwrap();
            let a = ProgramParser::new().parse(&s).unwrap();
            let type_res = type_checker(a.clone());
            for t in type_res {
                assert!( t.is_ok() ); 
            }
            assert!( interpreter(a.clone()).is_ok() );
        }
    }

    #[test]
    fn borrow_checking() {
        let mut dir = env::current_dir().unwrap();
        dir.push("src");
        dir.push("tests");
        dir.push("borrow_checking");
        let paths =fs::read_dir(dir);
        for file in paths.unwrap() {
            let s = fs::read_to_string(file.unwrap().path()).unwrap();
            let a = ProgramParser::new().parse(&s).unwrap();
            let type_res = type_checker(a.clone());
            for t in type_res {
                assert!( t.is_ok() ); 
            }
            assert!( interpreter(a.clone()).is_ok() );
        }
    }
    #[test]
    fn borrow_checking_errors() {
        let mut dir = env::current_dir().unwrap();
        dir.push("src");
        dir.push("tests");
        dir.push("borrow_checking_errors");
        let paths =fs::read_dir(dir);
        for file in paths.unwrap() {
            let s = fs::read_to_string(file.unwrap().path()).unwrap();
            let a = ProgramParser::new().parse(&s).unwrap();
            let type_res = type_checker(a.clone());
            for t in type_res {
                assert!( t.is_ok() );
            }
            assert!( interpreter(a.clone()).is_err() );
        }
    }

    #[test]
    fn type_checking() {
        let mut dir = env::current_dir().unwrap();
        dir.push("src");
        dir.push("tests");
        dir.push("type_checking");
        let paths =fs::read_dir(dir);
        for file in paths.unwrap() {
            let s = fs::read_to_string(file.unwrap().path()).unwrap();
            let a = ProgramParser::new().parse(&s).unwrap();
            let type_res = type_checker(a.clone());
            for t in type_res {
                assert!( t.is_ok() );
            }
        }
    }

    #[test]
    fn type_checking_rejections() {
        let mut dir = env::current_dir().unwrap();
        dir.push("src");
        dir.push("tests");
        dir.push("type_checking_rejections");
        let paths =fs::read_dir(dir);
        for file in paths.unwrap() {
            let s = fs::read_to_string(file.unwrap().path()).unwrap();
            let a = ProgramParser::new().parse(&s).unwrap();
            let type_res = type_checker(a.clone());
            for t in type_res {
                assert!( t.is_err() );
            }
        }
    }

    #[test]
    fn correct_parsing() {
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
        assert!(ProgramParser::new().parse("if 5 {};").is_ok());
        assert!(ProgramParser::new().parse("-------5").is_ok());
        assert!(ProgramParser::new().parse("******5").is_ok());
        assert!(ProgramParser::new().parse("let a:& & &i32 = & & &5;").is_ok());
    }

    #[test]
    fn failed_parsing() {
        assert!(ProgramParser::new().parse("fn a() -> () {a=51};").is_err());
        assert!(ProgramParser::new().parse("fn a() -> () {if {};};").is_err());
        assert!(ProgramParser::new().parse("fn a() -> () {let 8:bool = false;};").is_err());
        assert!(ProgramParser::new().parse("fn a() -> () {}").is_err());
        assert!(ProgramParser::new().parse("fn a() -> () {let a:&&&i32 = &&&5;};").is_err());
        assert!(ProgramParser::new().parse("fn a() -> () {a+=a};").is_err());
        assert!(ProgramParser::new().parse("fn a() {};").is_err());
        assert!(ProgramParser::new().parse("5/++++1321").is_err());
        assert!(ProgramParser::new().parse("while true").is_err());
        assert!(ProgramParser::new().parse("if true {}").is_err());
    }
}
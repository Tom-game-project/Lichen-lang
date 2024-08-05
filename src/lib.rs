mod parser;
//use crate::parser::core::ExprParser;
use parser::core_parser::Parser;
use parser::state_parser::StateParser;

mod abs;
mod token;

// test case
#[cfg(test)]
mod tests {

    use std::string;

    use crate::{parser::expr_parser, Parser, StateParser};

    #[test]
    fn test00() {
        let program = String::from("[[0,1,2],[4,5,6]]{a{123\"456\"}\"42\"{hello}}world(hello)");
        let parser = StateParser::new(program.clone(), 0, 0);

        println!("test case {}", program);
        let rlst = parser.resolve();

        if let Ok(v) = rlst {
            for i in v {
                i.show();
            }
        } else if let Err(e) = rlst {
            println!("{}", e);
        };

        //println!("{:?}",rlst);
        //assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test01() {
        let program = String::from(
            "fn add(a:i32,b:i32):i32{
    return a + b;
}

pub fn sub(a:i32,b:i32):i32{
    let c = a - b;
    return c;
}

pub fn main (a:i32,b:i32):void{
    let c = add(1,2);
    let d:i32 = a / (b*(c+d));
    c += 1;
    d = d + 42;
    return d;
}

pub fn up(a:i32,b:i32):(i32,i32){
    if (a <= b){
        return a,b;
    }else{
        return b,a;
    }
}",
        );
        let parser = StateParser::new(program.clone(), 0, 0);

        println!(
            "----------------------test case-----------------------
{}
------------------------------------------------------
",
            program
        );
        let rlst = parser.resolve();

        match rlst {
            Ok(v) => {
                for i in v {
                    i.show();
                }
            }
            Err(e) => {
                println!("error msg {}", e);
            }
        }

        //println!("{:?}",rlst);
        //assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test02() {
        println!("{}", "@".repeat(5));
    }

    // expr tests

    /// # expr_test00
    /// 式を正しくIR(中間形式)に変換できるかどうかのテスト
    #[test]
    fn expr_test00() {
        let code = "myfunc(0,1) + 2 * x";
        let string_code: String = String::from(code);
        let e_parser = expr_parser::ExprParser::new(string_code, 0, 0);

        let a = e_parser.resolve(); // 式解釈
        if let Ok(e) = a {
            for i in e {
                i.show()
            }
        }
    }
}

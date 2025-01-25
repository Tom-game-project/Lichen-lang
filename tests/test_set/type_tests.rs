extern crate lichen_lang;

use lichen_lang::parser::core_parser::Parser;
use lichen_lang::parser::type_parser::TypeParser;


/// 型を正しくパースできているかをチェックします
///
#[test]
pub fn type_test00(){
    let test_case: String = String::from("i32");
    let mut t_parser = TypeParser::new(
        test_case, 0, 0);

    if let Err(e) = t_parser.resolve() {
        println!("unexpected ParseError occured");
        println!("{:?}", e);
        panic!()
    } else {
        println!("{:?}", t_parser.code_list);
    }
}


/// 型を正しくパースできているかをチェックします
///
#[test]
pub fn type_test01(){
    let test_cases:Vec<&str> = vec![
        "i32",
        "bool",
        "Vec(i32)",
        "Option(i32, f32)",
        "(i32)",
        "(i32, i32)",
        "(i32,i32,i32)",
        "(Vec(i32), i32, i32)",
        "Vec(i32)",
        "(i32)->bool",
        //"(a:i32, b:i32)",
    ];

    for test_case in test_cases {
        let mut t_parser = TypeParser::new(
            test_case.to_string(), 0, 0);

        println!("test case ------- {}", test_case);
        if let Err(e) = t_parser.resolve() {
            println!("unexpected ParseError occured");
            println!("{:?}", e);
            panic!()
        } else {
            println!("{:?}", t_parser.code_list);
        }
    }
}


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

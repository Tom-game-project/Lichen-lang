use crate::abs::ast::*;
use crate::parser::type_parser::TypeParser;
use crate::errors::parser_errors::ParserError;

// `->`
// 型表現のために扱う演算子を格納します
#[derive(Clone, Debug)]
pub struct TypeOpeBranch {
    pub name: String,
    pub depth:isize,
    pub loopdepth:isize,
}

impl ASTBranch for TypeOpeBranch {
    fn show(&self) {
        println!("ope {}", self.name);
    }

    fn get_show_as_string(&self) -> String {
        format!("ope {}\n", self.name)
    }
}

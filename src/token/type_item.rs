use crate::abs::ast::*;
use crate::abs::gen::Wasm_gen;
use crate::errors::generate_errors::GenerateError;
use crate::errors::parser_errors::ParserError;
use crate::gen::wasm::MEMORY_SPACE_NAME;
use crate::parser::expr_parser::ExprParser;

/// 引数などの式を格納します
#[derive(Clone, Debug)]
pub struct TypeItemBranch {
    pub contents: Vec<TypeElem>,
    pub depth: isize,
    pub loopdepth: isize,
}


impl ASTBranch for TypeItemBranch {
    fn show(&self) {
        println!("{}Item(", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            i.show();
        }
        println!("{})", " ".repeat(self.depth as usize * 4));

    }

    fn get_show_as_string(&self) -> String {
        let mut rstr = format!("{}Item(\n", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            rstr = format!("{}{}", rstr, i.get_show_as_string());
        }
        rstr = format!("{}\n{})", rstr, " ".repeat(self.depth as usize * 4));
        rstr

    }
}

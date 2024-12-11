/// 関数を定義の要素

use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;

use super::block::BlockBranch;

#[derive(Clone, Debug)]
pub struct DecFuncBranch {
    pub func_name: String,
    pub arg_types: Vec<TypeElem>,
    pub return_types: Vec<TypeElem>,
    pub contents: BlockBranch,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for DecFuncBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        self.contents.resolve_self()?;
        Ok(())
    }
}

impl ASTBranch for DecFuncBranch{
    fn show(&self) {
        println!("function name \"{}\"", self.func_name);
        println!("arg_types{:?}", self.arg_types);
        println!("return types{:?}", self.return_types);
        println!("content vvv");
        self.contents.show();
    }

    fn get_show_as_string(&self) -> String {
        let mut rstr = String::default();

        rstr.push_str(&format!("func name\"{}\"\n", self.func_name));
        rstr.push_str(&format!("arg types{:?}\n",self.arg_types));
        rstr.push_str(&format!("return types{:?}\n",self.return_types));
        rstr.push_str("content vvv:word\n");
        rstr.push_str( &self.contents.get_show_as_string());

        rstr
    }
}


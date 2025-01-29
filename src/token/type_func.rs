// `->` を格納する
//
// 右優先
// ```
// T -> ( T -> ( T -> T ) )
// ```
//

use crate::abs::ast::*;
use crate::parser::type_parser::TypeParser;
use crate::errors::parser_errors::ParserError;

/// `<ltype>` `->` `<rtype>`
#[derive(Clone, Debug)]
pub struct TypeArrowBranch {
    pub ltype: Box<TypeElem>,
    pub rtype: Box<TypeElem>,
    pub depth:isize,
    pub loopdepth:isize,
}

impl ASTBranch for  TypeArrowBranch{
    fn show(&self) {
        //
        println!("{}", self.get_show_as_string());
    }

    fn get_show_as_string(&self) -> String {
        format!("{} -> {}", self.ltype.get_show_as_string(), self.ltype.get_show_as_string())
    }
}

impl RecursiveAnalysisTypeElements for TypeArrowBranch {
    fn resolve_self_as_type(&mut self) -> Result<(), ParserError> {
        self.ltype.resolve_self()?;
        self.rtype.resolve_self()?;
        Ok(())
    }
}


use crate::abs::ast::*;
use crate::parser::parser_errors::ParserError;
use crate::token::paren_block::ParenBlockBranch;

/// # FuncBranch
/// 関数呼び出しのトークン
/// ```
/// f(args)
/// ```
#[derive(Clone)]
pub struct FuncBranch {
    pub name: Box<BaseElem>,
    pub contents: ParenBlockBranch,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for FuncBranch {
    fn show(&self) {
        todo!();
    }
}

impl RecursiveAnalysisElements for FuncBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}

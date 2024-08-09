use crate::abs::ast::*;
use crate::token::paren_block::ParenBlockBranch;

/// # FuncBranch
/// 関数宣言を探す
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

impl ASTAreaBranch for FuncBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        todo!()
    }
    // fn resolve_self(&mut self) -> Result<&str, String> {
    //     todo!()
    // }
}

use crate::abs::ast::*;
/// #OperatorBranch
/// 全ての演算子
#[derive(Clone)]
pub struct OperatorBranch {
    pub ope: String,
    pub depth: isize,
}

impl ASTBranch for OperatorBranch {
    fn show(&self) {
        println!("Operator {}({})", self.depth, self.ope);
    }
}

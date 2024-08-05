use crate::abs::ast::*;
/// #OperatorBranch
/// 全ての演算子
#[derive(Clone)]
pub struct OperatorBranch {
    pub ope: String,
    pub depth: isize,
}

/// # StringBranch
#[derive(Clone)]
pub struct StringBranch {
    pub contents: String,
    pub depth: isize,
}

impl ASTBranch for StringBranch {
    fn show(&self) {
        println!("String ({})", self.contents);
        println!(
            "{}String ({})",
            " ".repeat(self.depth as usize),
            self.contents
        );
    }
}

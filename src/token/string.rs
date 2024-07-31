use crate::abs::ast::*;

/// # WordBranch
/// 単語を格納するためのstruct
/// ASTAreaBranchを実装しないため`resolve_self`メソッドを持たない
#[derive(Clone)]
pub struct StringBranch {
    pub contents: String,
    pub depth: isize,
}

impl ASTBranch for StringBranch {
    fn show(&self) {
        println!("Word {}", self.contents)
    }
}

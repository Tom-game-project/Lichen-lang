use crate::abs::ast::*;

/// # WordBranch
/// 単語を格納するためのstruct
/// ASTAreaBranchを実装しないため`resolve_self`メソッドを持たない
#[derive(Clone)]
pub struct WordBranch {
    pub contents: String,
}

impl ASTBranch for WordBranch {
    fn show(&self) {
        println!("Word {}", self.contents)
    }
}

use crate::abs::ast::*;

/// # UnKnownBranch
///未定トークンが以下のstructに分類される
#[derive(Clone)]
pub struct UnKnownBranch {
    pub contents: char,
}

impl ASTBranch for UnKnownBranch {
    fn show(&self) {
        println!("UnKnownBranch :\"{}\"", self.contents);
    }
}

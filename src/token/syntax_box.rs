use crate::abs::ast::*;
use crate::parser::parser_errors::ParserError;
use crate::token::syntax::SyntaxBranch;

/// # SyntaxBoxBranch
/// まとまった文法として解釈される`if elif else` `while else` `for else`などの文法をまとめる
#[derive(Clone)]
pub struct SyntaxBoxBranch {
    pub name: String,
    pub contents: Vec<SyntaxBranch>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for SyntaxBoxBranch {
    fn show(&self) {
        todo!()
    }
}

impl RecursiveAnalysisElements for SyntaxBoxBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}

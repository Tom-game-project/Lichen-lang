use crate::abs::ast::*;

use crate::parser::parser_errors::ParserError;

/// #ListBlockBranch
/// listを格納するためのデータstruct
/// 中では式を解析するパーサを呼び出す必要がある
#[derive(Clone)]
pub struct ListBlockBranch {
    pub contents: Option<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for ListBlockBranch {
    fn show(&self) {
        println!(
            "{}List depth{} (",
            " ".repeat(self.depth as usize),
            self.depth
        );
        if let Some(e) = &self.contents {
            for i in e {
                i.show();
            }
        }
        println!(")");
    }
}

impl ASTAreaBranch for ListBlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents: contents,
            depth: depth,
            loopdepth: loopdepth,
        }
    }
}

impl RecursiveAnalysisElements for ListBlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}

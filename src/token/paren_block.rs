use crate::abs::ast::*;
use crate::parser::parser_errors::ParserError;

/// #ParenBlockBranch
/// `()`を使用したプログラムにおけるデータを格納するstruct
/// 中では,
/// - 式を解析する必要がある場合
/// - タイプ宣言を解析する必要がある場合１ ex) (a:T, b:T)
/// - タイプ宣言を解析する必要がある場合２ ex) (T, T)
/// があり個別に呼び出すパーサを実装する必要がある。
/// 実装する
#[derive(Clone)]
pub struct ParenBlockBranch {
    pub code_list: Vec<BaseElem>,
    pub contents: Option<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for ParenBlockBranch {
    fn show(&self) {
        println!(
            "{}Paren depth{} (",
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

impl ASTAreaBranch for ParenBlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        Self {
            code_list: Vec::new(),
            contents: contents,
            depth: depth,
            loopdepth: loopdepth,
        }
    }
}

impl RecursiveAnalysisElements for ParenBlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        for inner in &mut self.code_list {
            if let Err(e) = inner.resolve_self() {
                return Err(e);
            }
        }
        Ok(())
    }
}

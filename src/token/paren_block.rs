use crate::abs::ast::*;
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
            contents: contents,
            depth: depth,
            loopdepth: loopdepth,
        }
    }
    // fn resolve_self(&mut self) -> Result<&str, String> {
    //     // TODO: impl expr parser
    //     // TODO: impl args parser
    //     // TODO: impl tuple parser
    //     return Ok("Ok!");
    // }
}

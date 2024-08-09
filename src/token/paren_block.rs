use crate::abs::ast::*;
use crate::parser::expr_parser::ExprParser;
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
    pub contents: Option<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for ParenBlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        // 式パーサーによって解析
        if let Some(a) = &self.contents {
            let mut parser =
                ExprParser::create_parser_from_vec(a.to_vec(), self.depth + 1, self.loopdepth);
            match parser.code2vec2() {
                Ok(_) => {
                    let mut rlist = parser.code_list;
                    for i in &mut rlist {
                        if let Err(e) = i.resolve_self() {
                            return Err(e);
                        }
                    }
                    self.contents = Some(rlist);
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
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
}

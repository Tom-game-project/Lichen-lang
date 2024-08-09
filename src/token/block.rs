use crate::abs::ast::*;

use crate::parser::parser_errors::ParserError;
use crate::parser::state_parser::*;

/// # BlockBranch
/// ブロックを格納するデータstruct
/// 内部では文を解析するパーサを呼び出す必要がある
///
#[derive(Clone)]
pub struct BlockBranch {
    pub contents: Option<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for BlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        if let Some(a) = &self.contents {
            // let parser = StateParser::new(String::from(""), self.depth + 1, self.loopdepth);
            let mut parser =
                StateParser::create_parser_from_vec(a.to_vec(), self.depth + 1, self.loopdepth);
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
        } else {
            return Ok(());
        }
    }
}

impl ASTAreaBranch for BlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents: contents,
            depth: depth,
            loopdepth: loopdepth,
        }
    }
}

impl ASTBranch for BlockBranch {
    fn show(&self) {
        println!(
            "{}BlockBranch depth{} (",
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

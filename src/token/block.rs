use crate::abs::ast::*;

use crate::parser::core_parser::*;
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

impl ASTAreaBranch for BlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents: contents,
            depth: depth,
            loopdepth: loopdepth,
        }
    }

    fn resolve_self(&mut self) -> Result<&str, String> {
        if let Some(a) = &self.contents {
            let parser = StateParser::new(String::from(""), self.depth + 1, self.loopdepth);
            match parser.code2vec(&a) {
                Ok(v) => {
                    let mut rlist = v.to_vec();
                    for i in &mut rlist {
                        match i.resolve_self() {
                            Ok(_) => { /* pass */ }
                            Err(_) => { /* pass */ }
                        };
                    }
                    self.contents = Some(rlist);
                    return Ok("OK!");
                }
                Err(e) => {
                    // println!("{}",e);
                    return Err(String::from(e));
                }
            }
        } else {
            return Ok("Empty");
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

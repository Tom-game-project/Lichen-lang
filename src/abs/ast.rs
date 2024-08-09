use crate::token::{
    block::BlockBranch, func::FuncBranch, list_block::ListBlockBranch, operator::OperatorBranch,
    paren_block::ParenBlockBranch, string::StringBranch, syntax::SyntaxBranch,
    syntax_box::SyntaxBoxBranch, unknown::UnKnownBranch, word::WordBranch,
};

use crate::parser::parser_errors::ParserError;

/// # BaseElem
/// 抽象的なtoken
/// プログラムの要素を表現できる
#[derive(Clone)]
pub enum BaseElem {
    BlockElem(BlockBranch),
    ListBlockElem(ListBlockBranch),
    ParenBlockElem(ParenBlockBranch),
    SyntaxElem(SyntaxBranch),
    SyntaxBoxElem(SyntaxBoxBranch),
    FuncElem(FuncBranch),
    // without ASTAreaBranch trait structures
    StringElem(StringBranch),
    WordElem(WordBranch),
    OpeElem(OperatorBranch),
    UnKnownElem(UnKnownBranch),
}

impl BaseElem {
    pub fn show(&self) {
        match self {
            BaseElem::BlockElem(e) => e.show(),
            BaseElem::UnKnownElem(e) => e.show(),
            BaseElem::StringElem(e) => e.show(),
            BaseElem::ListBlockElem(e) => e.show(),
            BaseElem::ParenBlockElem(e) => e.show(),
            BaseElem::WordElem(e) => e.show(),
            BaseElem::SyntaxElem(e) => e.show(),
            BaseElem::SyntaxBoxElem(e) => e.show(),
            BaseElem::FuncElem(e) => e.show(),
            BaseElem::OpeElem(e) => e.show(),
        }
    }

    pub fn resolve_self(&mut self) -> Result<(), ParserError> {
        match self {
            // recursive analysis elements
            BaseElem::BlockElem(e) => return e.resolve_self(),
            BaseElem::ListBlockElem(e) => return e.resolve_self(),
            BaseElem::ParenBlockElem(e) => return e.resolve_self(),
            BaseElem::SyntaxElem(e) => return e.resolve_self(),
            BaseElem::SyntaxBoxElem(e) => return e.resolve_self(),
            BaseElem::FuncElem(e) => return e.resolve_self(),

            // unrecursive analysis elements
            BaseElem::StringElem(_) => return Ok(()),
            BaseElem::WordElem(_) => return Ok(()),
            BaseElem::OpeElem(_) => return Ok(()),
            BaseElem::UnKnownElem(_) => return Ok(()),
        }
    }
}

/// #  ASTBranch
/// このtraitを実装している構造体は
/// - 自分自身の構造をわかりやすく出力できる
pub trait ASTBranch {
    fn show(&self);
}

/// # ASTAreaBranch
/// ## resolve_self
/// depthをインクリメントするときは、`resolve_self`内で宣言するParserにself.get_depth + 1をして実装する必要がある
pub trait ASTAreaBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self;
    // fn resolve_self(&mut self) -> Result<&str, String>;
}

pub trait RecursiveAnalysisElements {
    fn resolve_self(&mut self) -> Result<(), ParserError>;
}

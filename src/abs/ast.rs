// tokens
use crate::token::block::BlockBranch;
use crate::token::comment::CommentBranch;
use crate::token::decfunc::DecFuncBranch;
use crate::token::func::FuncBranch;
use crate::token::item::ItemBranch;
use crate::token::list::ListBranch;
use crate::token::list_block::ListBlockBranch;
use crate::token::operator::OperatorBranch;
use crate::token::paren_block::ParenBlockBranch;
use crate::token::stmt::expr::ExprBranch;
use crate::token::stmt::stmt::StmtBranch;
use crate::token::string::StringBranch;
use crate::token::syntax::SyntaxBranch;
use crate::token::syntax_box::SyntaxBoxBranch;

use crate::token::unknown::UnKnownBranch;
use crate::token::word::WordBranch;
use crate::token::type_item::TypeItemBranch;
use crate::token::type_block::TypeBlockBranch;
use crate::token::type_ope::TypeOpeBranch;
// errors
use crate::errors::parser_errors::ParserError;


pub trait Token {
    fn set_char_as_unknown(c: char) -> Self;
    fn show(&self);
    fn get_show_as_string(&self) -> String;
    fn resolve_self(&mut self) -> Result<(), ParserError>;
}

pub trait ProcToken {
    // `ExprElem`と`StmtElem`に実装
    // 解析時に揺れがある
    fn t_string(contents: String, depth: isize, loopdepth: isize) -> Self;
    fn t_block(contents: Vec<StmtElem>, depth: isize, loopdepth: isize) -> Self;
    fn t_parenblock(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self;
    fn t_listblock(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self;
    fn t_commentblock(contents: String, depth: isize, loopdepth: isize) -> Self;
}

/// # ExprElem
/// 抽象的なtoken
/// プログラムの要素を表現できる
#[derive(Clone, Debug)]
pub enum ExprElem {
    BlockElem(BlockBranch),
    ListBlockElem(ListBlockBranch),
    ParenBlockElem(ParenBlockBranch),
    SyntaxElem(SyntaxBranch),
    SyntaxBoxElem(SyntaxBoxBranch),
    FuncElem(FuncBranch),
    ListElem(ListBranch),
    ItemElem(ItemBranch),
    // without RecursiveAnalysisElements trait structures
    CommentElem(CommentBranch),
    StringElem(StringBranch),
    WordElem(WordBranch),
    OpeElem(OperatorBranch),
    UnKnownElem(UnKnownBranch),
}

#[derive(Clone, Debug)]
pub enum TypeElem {
    CommentElem(CommentBranch),
    WordElem(WordBranch),
    ParenBlockElem(ParenBlockBranch),
    ListBlockElem(ListBlockBranch),
    TypeBlockElem(TypeBlockBranch),
    ItemBlockElem(TypeItemBranch),
    TypeFuncElem(TypeBlockBranch),
    TypeOpeElem(TypeOpeBranch),
    UnKnownElem(UnKnownBranch),
}

#[derive(Clone, Debug)]
pub enum StmtElem {
    BlockElem(BlockBranch),
    ListBlockElem(ListBlockBranch),
    ParenBlockElem(ParenBlockBranch),
    //
    ExprElem(ExprBranch),
    Special(StmtBranch),
    DefineElem(DecFuncBranch),
    // without RecursiveAnalysisElements trait structures
    StringElem(StringBranch),
    WordElem(WordBranch),
    OpeElem(OperatorBranch),
    CommentElem(CommentBranch),
    UnKnownElem(UnKnownBranch),
}

impl Token for ExprElem {
    fn set_char_as_unknown(c: char) -> Self {
        ExprElem::UnKnownElem(UnKnownBranch { contents: c })
    }

    fn show(&self) {
        match self {
            Self::BlockElem(e) => e.show(),
            Self::UnKnownElem(e) => e.show(),
            Self::StringElem(e) => e.show(),
            Self::ListBlockElem(e) => e.show(),
            Self::ParenBlockElem(e) => e.show(),
            Self::WordElem(e) => e.show(),
            Self::SyntaxElem(e) => e.show(),
            Self::SyntaxBoxElem(e) => e.show(),
            Self::FuncElem(e) => e.show(),
            Self::ItemElem(e) => e.show(),
            Self::OpeElem(e) => e.show(),
            Self::ListElem(e) => e.show(),
            Self::CommentElem(e) => e.show(),
        }
    }

    fn get_show_as_string(&self) -> String {
        match self {
            Self::BlockElem(e) => e.get_show_as_string(),
            Self::UnKnownElem(e) => e.get_show_as_string(),
            Self::StringElem(e) => e.get_show_as_string(),
            Self::ListBlockElem(e) => e.get_show_as_string(),
            Self::ParenBlockElem(e) => e.get_show_as_string(),
            Self::WordElem(e) => e.get_show_as_string(),
            Self::SyntaxElem(e) => e.get_show_as_string(),
            Self::SyntaxBoxElem(e) => e.get_show_as_string(),
            Self::FuncElem(e) => e.get_show_as_string(),
            Self::ItemElem(e) => e.get_show_as_string(),
            Self::OpeElem(e) => e.get_show_as_string(),
            Self::ListElem(e) => e.get_show_as_string(),
            Self::CommentElem(e) => e.get_show_as_string(),
        }
    }

    fn resolve_self(&mut self) -> Result<(), ParserError> {
        match self {
            // recursive analysis elements
            Self::BlockElem(e) => e.resolve_self(),
            Self::ListBlockElem(e) => e.resolve_self(),
            Self::ParenBlockElem(e) => e.resolve_self(),
            Self::SyntaxElem(e) => e.resolve_self(),
            Self::SyntaxBoxElem(e) => e.resolve_self(),
            Self::FuncElem(e) => e.resolve_self(),
            Self::ListElem(e) => e.resolve_self(),
            Self::ItemElem(e) => e.resolve_self(),

            // unrecursive analysis elements
            Self::StringElem(_) => Ok(()),
            Self::WordElem(_) => Ok(()),
            Self::OpeElem(_) => Ok(()),
            Self::UnKnownElem(_) => Ok(()),
            Self::CommentElem(_) => Ok(()),
        }
    }
}

impl ProcToken for ExprElem {
    fn t_string(contents: String, depth: isize, loopdepth: isize) -> Self {
        Self::StringElem(StringBranch {
            contents,
            depth,
            loopdepth,
        })
    }

    fn t_block(contents: Vec<StmtElem>, depth: isize, loopdepth: isize) -> Self {
        Self::BlockElem(BlockBranch {
            contents,
            depth,
            loopdepth,
        })
    }

    fn t_parenblock(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self {
        Self::ParenBlockElem(ParenBlockBranch {
            contents,
            contents_as_type: Vec::default(),
            depth,
            loopdepth,
        })
    }

    fn t_listblock(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self {
        Self::ListBlockElem(ListBlockBranch {
            contents,
            depth,
            loopdepth,
        })
    }
    fn t_commentblock(contents: String, depth: isize, loopdepth: isize) -> Self {
        Self::CommentElem(CommentBranch {
            contents,
            depth,
            loopdepth,
        })
    }
}

impl Token for TypeElem {
    fn set_char_as_unknown(c: char) -> Self {
        TypeElem::UnKnownElem(UnKnownBranch { contents: c })
    }

    fn get_show_as_string(&self) -> String {
        match self {
            TypeElem::WordElem(e) => e.get_show_as_string(),
            TypeElem::TypeBlockElem(e) => e.get_show_as_string(),
            TypeElem::TypeFuncElem(e) => e.get_show_as_string(),
            TypeElem::UnKnownElem(e) => e.get_show_as_string(),
            TypeElem::CommentElem(e) => e.get_show_as_string(),
            TypeElem::ListBlockElem(e) => e.get_show_as_string(),
            TypeElem::ParenBlockElem(e) => e.get_show_as_string(),
            TypeElem::ItemBlockElem(e) => e.get_show_as_string(),
            TypeElem::TypeOpeElem(e) => e.get_show_as_string(),
        }
    }

    fn show(&self) {
        match self {
            TypeElem::WordElem(e) => e.show(),
            TypeElem::TypeBlockElem(e) => e.show(),
            TypeElem::TypeFuncElem(e) => e.show(),
            TypeElem::UnKnownElem(e) => e.show(),
            TypeElem::CommentElem(e) => e.show(),
            TypeElem::ListBlockElem(e) => e.show(),
            TypeElem::ParenBlockElem(e) => e.show(),
            TypeElem::ItemBlockElem(e) => e.show(),
            TypeElem::TypeOpeElem(e) => e.show(),
        }
    }

    fn resolve_self(&mut self) -> Result<(), ParserError> {
        // resolve_self_as_typeというmethodを呼び出したい
        match self {
            TypeElem::ParenBlockElem(pb) =>{
                pb.resolve_self_as_type()
            },
            TypeElem::TypeBlockElem(tb) =>{
                todo!()
            },
            TypeElem::TypeFuncElem(tfb) => {
                tfb.resolve_self_as_type()
            },
            TypeElem::ItemBlockElem(ib) =>{
                ib.resolve_self_as_type()
            },
            TypeElem::WordElem(_wb) => Ok(()),
            TypeElem::TypeOpeElem(_ob) => Ok(()),
            TypeElem::CommentElem(_cb) => Ok(()),
            _ => {
                // 最終的にこのブロックはなくす
                println!("{:?}", self);
                todo!()
            }
        }
    }
}

impl Token for StmtElem {
    fn set_char_as_unknown(c: char) -> Self {
        StmtElem::UnKnownElem(UnKnownBranch { contents: c })
    }

    fn get_show_as_string(&self) -> String {
        match self {
            Self::BlockElem(e) => e.get_show_as_string(),
            Self::ListBlockElem(e) => e.get_show_as_string(),
            Self::ParenBlockElem(e) => e.get_show_as_string(),
            Self::Special(e) => e.get_show_as_string(),
            Self::DefineElem(e) => e.get_show_as_string(),
            // without RecursiveAnalysisElements trait structures
            Self::StringElem(e) => e.get_show_as_string(),
            Self::CommentElem(e) => e.get_show_as_string(),
            Self::ExprElem(e) => e.get_show_as_string(),
            Self::WordElem(e) => e.get_show_as_string(),
            Self::OpeElem(e) => e.get_show_as_string(),
            Self::UnKnownElem(e) => e.get_show_as_string(),
        }
    }

    fn show(&self) {
        match self {
            Self::BlockElem(e) => e.show(),
            Self::ListBlockElem(e) => e.show(),
            Self::ParenBlockElem(e) => e.show(),
            Self::Special(e) => e.show(),
            Self::DefineElem(e) => e.show(),
            // without RecursiveAnalysisElements trait structures
            Self::StringElem(e) => e.show(),
            Self::CommentElem(e) => e.show(),
            Self::ExprElem(e) => e.show(),
            Self::WordElem(e) => e.show(),
            Self::OpeElem(e) => e.show(),
            Self::UnKnownElem(e) => e.show(),
        }
    }

    fn resolve_self(&mut self) -> Result<(), ParserError> {
        match self {
            Self::BlockElem(e) => e.resolve_self(),
            Self::ListBlockElem(e) => e.resolve_self(),
            Self::ParenBlockElem(e) => e.resolve_self(),
            Self::ExprElem(e) => e.resolve_self(),
            // Self::DefineElem(e) => 
            Self::Special(e) => e.resolve_self(),
            Self::DefineElem(e) => e.resolve_self(),
            // without RecursiveAnalysisElements trait structures
            Self::StringElem(_) => Ok(()),
            Self::CommentElem(_) => Ok(()),
            Self::WordElem(_) => Ok(()),
            Self::OpeElem(_) => Ok(()),
            Self::UnKnownElem(_) => Ok(()),
        }
    }
}

impl ProcToken for StmtElem {
    fn t_string(contents: String, depth: isize, loopdepth: isize) -> Self {
        Self::StringElem(StringBranch {
            contents,
            depth,
            loopdepth,
        })
    }

    fn t_block(contents: Vec<StmtElem>, depth: isize, loopdepth: isize) -> Self {
        Self::BlockElem(BlockBranch {
            contents,
            depth,
            loopdepth,
        })
    }

    fn t_parenblock(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self {
        Self::ParenBlockElem(ParenBlockBranch {
            contents,
            contents_as_type: Vec::default(),
            depth,
            loopdepth,
        })
    }

    fn t_listblock(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self {
        Self::ListBlockElem(ListBlockBranch {
            contents,
            depth,
            loopdepth,
        })
    }

    fn t_commentblock(contents: String, depth: isize, loopdepth: isize) -> Self {
        Self::CommentElem(CommentBranch {
            contents,
            depth,
            loopdepth,
        })
    }
}

/// #  ASTBranch
/// token buranch should be implemented this trait
pub trait ASTBranch {
    fn show(&self);
    fn get_show_as_string(&self) -> String;
    //  fn expand_assembly(&self) -> String;
}

/// # ASTAreaBranch
/// ## resolve_self
/// depthをインクリメントするときは、`resolve_self`内で宣言するParserにself.get_depth + 1をして実装する必要がある
pub trait ASTAreaBranch<T>
where
    T: Token,
{
    fn new(contents: Vec<T>, depth: isize, loopdepth: isize) -> Self;
}

pub trait TypeAreaBranch {
    fn new(contents: Vec<TypeElem>, depth: isize) -> Self;
}


/// 再帰的にパースを必要とするプログラム片に実装する
pub trait RecursiveAnalysisElements {
    fn resolve_self(&mut self) -> Result<(), ParserError>;
}


/// 再帰的にパースを必要とする型宣言に実装する
pub trait RecursiveAnalysisTypeElements {
    fn resolve_self_as_type(&mut self) -> Result<(), ParserError>;
}

use crate::parser::core::*;

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
        }
    }

    pub fn resolve_self(&mut self) -> Result<&str, String> {
        match self {
            // recursive analysis elements
            BaseElem::BlockElem(e) => return e.resolve_self(),
            BaseElem::ListBlockElem(e) => return e.resolve_self(),
            BaseElem::ParenBlockElem(e) => return e.resolve_self(),
            BaseElem::SyntaxElem(e) => return e.resolve_self(),
            BaseElem::SyntaxBoxElem(e) => return e.resolve_self(),
            BaseElem::FuncElem(e) => return e.resolve_self(),

            // unrecursive analysis elements
            BaseElem::StringElem(_) => return Ok("Ok"),
            BaseElem::WordElem(_) => return Ok("Ok"),
            BaseElem::UnKnownElem(_) => return Ok("Ok"),
        }
    }
}

pub trait ASTBranch {
    fn show(&self);
}

/// # ASTAreaBranch
/// ## resolve_self
/// depthをインクリメントするときは、`resolve_self`内で宣言するParserにself.get_depth + 1をして実装する必要がある
pub trait ASTAreaBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self;
    fn resolve_self(&mut self) -> Result<&str, String>;
}

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

    fn resolve_self(&mut self) -> Result<&str, String> {
        //todo!();
        // TODO:impl list parser
        // TODO:impl slice parser
        return Ok("Ok!");
    }
}

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
    fn resolve_self(&mut self) -> Result<&str, String> {
        // TODO: impl expr parser
        // TODO: impl args parser
        // TODO: impl tuple parser
        return Ok("Ok!");
    }
}

/// #SyntaxBranch
/// `if` `else` `while` `loop` `for`などのデータを扱うstruct
/// resolve_selfはそれぞれ
/// `()`で格納されているデータに関しては`ParenBlockBranch`をnormalで呼び出す
/// `{}`で格納されているデータに関しては`BlockBranch`のパーサに丸投げする。
#[derive(Clone)]
pub struct SyntaxBranch {
    pub name: String,
    pub expr: Option<Box<BaseElem>>,
    pub contents: Option<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for SyntaxBranch {
    fn show(&self) {
        todo!()
    }
}

impl ASTAreaBranch for SyntaxBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        todo!()
    }

    fn resolve_self(&mut self) -> Result<&str, String> {
        todo!()
    }
}

/// # SyntaxBoxBranch
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

impl ASTAreaBranch for SyntaxBoxBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        todo!()
    }

    fn resolve_self(&mut self) -> Result<&str, String> {
        todo!()
    }
}

#[derive(Clone)]
pub struct FuncBranch {
    pub name: Box<BaseElem>,
    pub contents: ParenBlockBranch,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for FuncBranch {
    fn show(&self) {
        todo!();
    }
}

impl ASTAreaBranch for FuncBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        todo!()
    }
    fn resolve_self(&mut self) -> Result<&str, String> {
        todo!()
    }
}

// structures without ASTAreaBranch trait b

/// # StringBranch
#[derive(Clone)]
pub struct StringBranch {
    pub contents: String,
    pub depth: isize,
}

impl ASTBranch for StringBranch {
    fn show(&self) {
        println!("String ({})", self.contents);
        println!(
            "{}String ({})",
            " ".repeat(self.depth as usize),
            self.contents
        );
    }
}

/// # WordBranch
/// 単語を格納するためのstruct
/// ASTAreaBranchを実装しないため`resolve_self`メソッドを持たない
#[derive(Clone)]
pub struct WordBranch {
    pub contents: String,
}

impl ASTBranch for WordBranch {
    fn show(&self) {
        println!("Word {}", self.contents)
    }
}

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

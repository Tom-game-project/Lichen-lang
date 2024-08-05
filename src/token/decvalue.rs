use crate::abs::ast::BaseElem;

struct DecValueBranch {
    pub valuename: String, // TODO:ここはいずれ、パターンにしたい
    pub contents: Vec<BaseElem>,
    pub depth: isize,
    pub loopdepth: isize,
    // flags
    pub is_mutable: bool,
    pub is_public: bool,
    pub exported: bool,
}

impl DecValueBranch {} // TODO:

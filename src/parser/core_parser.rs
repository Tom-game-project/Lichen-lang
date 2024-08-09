// use crate::parser::token::*;
use crate::abs::ast::*;
use crate::token::{
    func::FuncBranch, string::StringBranch, unknown::UnKnownBranch, word::WordBranch,
};

/// # Parser trait
/// パーサのコア実装
pub trait Parser<'a> {
    // operators
    const OR: &'a str = "||";
    const AND: &'a str = "&&";
    const EQ: &'a str = "==";
    const NE: &'a str = "!=";
    const LT: &'a str = "<";
    const LE: &'a str = "<=";
    const GT: &'a str = ">";
    const GE: &'a str = ">=";
    const ADD: &'a str = "+";
    const SUB: &'a str = "-";
    const MUL: &'a str = "*";
    const DIV: &'a str = "/";
    const MOD: &'a str = "%";
    const DOT: &'a str = "@";

    const ASSIGNMENT: &'a str = "=";
    const ADDEQ: &'a str = "+=";
    const SUBEQ: &'a str = "-=";
    const MULEQ: &'a str = "*=";
    const DIVEQ: &'a str = "/=";
    const MODEQ: &'a str = "%=";

    const POW: &'a str = "**";
    const NOT: &'a str = "!";

    const LEFT_PRIORITY_LIST: [(&'a str, isize); 14] = [
        (Self::OR, -3),  // ||
        (Self::AND, -2), // &&
        // PRIORITY 0
        (Self::EQ, 0), // ==
        (Self::NE, 0), // !=
        (Self::LT, 0), // <
        (Self::LE, 0), // <=
        (Self::GT, 0), // >
        (Self::GE, 0), // >=
        // PRIORITY 1
        (Self::ADD, 1), // +
        (Self::SUB, 1), // -
        // PRIORITY 2
        (Self::MUL, 2), // *
        (Self::DIV, 2), // /
        (Self::MOD, 2), // %
        (Self::DOT, 2), // @
    ];
    const RIGHT_PRIORITY_LIST: [(&'a str, isize); 7] = [
        // PRIORITY -4
        (Self::ASSIGNMENT, -4), // =
        (Self::ADDEQ, -4),      // +=
        (Self::SUBEQ, -4),      // -=
        (Self::MULEQ, -4),      // *=
        (Self::DIVEQ, -4),      // /=
        (Self::MODEQ, -4),      // %=
        (Self::POW, 3),         // **
    ];
    const PREFIX_PRIORITY_LIST: [(&'a str, isize); 1] = [
        // PRIORITY -1
        (Self::NOT, -1), // !
    ];

    /// 演算子を文字列として長いものからの順番で並べたもの
    const LENGTH_ORDER_OPE_LIST: [&'a str; 22] = [
        Self::OR,         // ||
        Self::AND,        // &&
        Self::EQ,         // ==
        Self::NE,         // !=
        Self::LE,         // <=
        Self::GE,         // >=
        Self::ADDEQ,      // +=
        Self::SUBEQ,      // -=
        Self::MULEQ,      // *=
        Self::DIVEQ,      // /=
        Self::MODEQ,      // %=
        Self::POW,        // **
        Self::LT,         // <
        Self::GT,         // >
        Self::ADD,        // +
        Self::SUB,        // -
        Self::MUL,        // *
        Self::DIV,        // /
        Self::MOD,        // %
        Self::DOT,        // @
        Self::ASSIGNMENT, // =
        Self::NOT,        // !
    ];

    const SPLIT_CHAR: [char; 3] = [' ', '\t', '\n'];
    const EXCLUDE_WORDS: [char; 3] = [';', ':', ','];

    const SYNTAX_IF: &'a str = "if";
    const SYNTAX_ELIF: &'a str = "elif";
    const SYNTAX_ELSE: &'a str = "else";
    const SYNTAX_LOOP: &'a str = "loop";
    const SYNTAX_FOR: &'a str = "for";
    const SYNTAX_WHILE: &'a str = "while";
    const SYNTAX_MATCH: &'a str = "match";

    const SYNTAX_WORDS: [&'a str; 7] = [
        Self::SYNTAX_IF,    // if
        Self::SYNTAX_ELIF,  // elif
        Self::SYNTAX_ELSE,  // else
        Self::SYNTAX_LOOP,  // loop
        Self::SYNTAX_FOR,   // for
        Self::SYNTAX_WHILE, // while
        Self::SYNTAX_MATCH, // match
    ];
    const SYNTAX_WORDS_HEADS: [&'a str; 4] = [
        Self::SYNTAX_IF,    // if
        Self::SYNTAX_LOOP,  // loop
        Self::SYNTAX_FOR,   // for
        Self::SYNTAX_WHILE, // while
    ];
    const ESCAPECHAR: char = '\\';
    const FUNCTION: &'a str = "fn";
    const SEMICOLON: char = ';';
    const DOUBLE_QUOTATION: char = '"';
    const SINGLE_QUOTATION: char = '\'';

    const CONTROL_RETURN: &'a str = "return";
    const CONTROL_BREAK: &'a str = "break";
    const CONTROL_CONTINUE: &'a str = "continue";
    const CONTROL_ASSERT: &'a str = "assert";

    const CONTROL_STATEMENT: [&'a str; 4] = [
        Self::CONTROL_RETURN,   // return
        Self::CONTROL_BREAK,    // break
        Self::CONTROL_CONTINUE, // continue
        Self::CONTROL_ASSERT,   // assert
    ];

    const BLOCK_BRACE_OPEN: char = '{';
    const BLOCK_BRACE_CLOSE: char = '}';
    const BLOCK_PAREN_OPEN: char = '(';
    const BLOCK_PAREN_CLOSE: char = ')';
    const BLOCK_LIST_OPEN: char = '[';
    const BLOCK_LIST_CLOSE: char = ']';

    fn new(code: String, depth: isize, loopdepth: isize) -> Self;
    fn resolve(&self) -> Result<Vec<BaseElem>, &str>;
    fn code2vec(&self, code: &Vec<BaseElem>) -> Result<Vec<BaseElem>, &str>;
    fn get_depth(&self) -> isize;
    fn get_loopdepth(&self) -> isize;

    fn code2_vec_pre_proc_func(&self, code: &String) -> Vec<BaseElem> {
        return code
            .chars()
            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
            .collect();
    }

    // grouoping functions
    fn grouping_quotation(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        let mut open_flag = false;
        let mut escape_flag = false;
        let mut rlist = Vec::new();
        let mut group = String::new();

        for inner in codelist {
            if let BaseElem::UnKnownElem(ref v) = inner {
                if escape_flag {
                    group.push(v.contents);
                    escape_flag = false
                } else {
                    if v.contents == '"'
                    // is quochar
                    {
                        if open_flag {
                            group.push(v.contents);
                            rlist.push(BaseElem::StringElem(StringBranch {
                                contents: group.clone(),
                                depth: self.get_depth(),
                            }));
                            group.clear();
                            open_flag = false;
                        } else {
                            group.push(v.contents);
                            open_flag = true;
                        }
                    } else {
                        if open_flag {
                            if v.contents == '\\' {
                                escape_flag = true;
                            } else {
                                escape_flag = false;
                            }
                            group.push(v.contents);
                        } else {
                            rlist.push(inner);
                        }
                    }
                }
            } else {
                rlist.push(inner);
            }
        }
        if open_flag {
            return Err("[Error: quotation is not closed]");
        }
        return Ok(rlist);
    }

    fn grouping_elements<T>(
        &self,
        codelist: Vec<BaseElem>,
        elemtype: fn(T) -> BaseElem,
        open_char: char,
        close_char: char,
    ) -> Result<Vec<BaseElem>, &str>
    where
        T: ASTAreaBranch,
    {
        let mut rlist: Vec<BaseElem> = Vec::new();
        let mut group: Vec<BaseElem> = Vec::new();
        let mut depth: isize = 0;

        for inner in codelist {
            if let BaseElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    if depth > 0 {
                        group.push(inner);
                    } else if depth == 0 {
                        // pass
                    } else {
                        return Err("[Error:]");
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    if depth > 0 {
                        group.push(inner);
                    } else if depth == 0 {
                        rlist.push(elemtype(ASTAreaBranch::new(
                            Some(group.clone()),
                            self.get_depth(),
                            self.get_loopdepth(),
                        )));
                        group.clear();
                    } else {
                        return Err("[Error:]");
                    }
                } else {
                    if depth > 0 {
                        group.push(inner);
                    } else if depth == 0 {
                        rlist.push(inner);
                    } else {
                        return Err("[Error:]");
                    }
                }
            } else {
                if depth > 0 {
                    group.push(inner);
                } else if depth == 0 {
                    rlist.push(inner);
                } else {
                    return Err("[Error:(user error) block must be closed]");
                }
            }
        }
        if depth != 0 {
            return Err("[Error:(user error) block must be closed]");
        }
        return Ok(rlist);
    }

    /// # grouping_word
    /// スペースなどので区切られた単語をまとめる
    fn grouping_word(
        &self,
        codelist: Vec<BaseElem>,
        split: Vec<char>,
        excludes: Vec<char>,
    ) -> Result<Vec<BaseElem>, &str> {
        let mut rlist: Vec<BaseElem> = Vec::new();
        let mut group: String = String::new();

        for inner in codelist {
            if let BaseElem::UnKnownElem(ref e) = inner {
                if split.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        rlist.push(BaseElem::WordElem(WordBranch {
                            contents: group.clone(),
                        }));
                        group.clear();
                    }
                } else if excludes.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        rlist.push(BaseElem::WordElem(WordBranch {
                            contents: group.clone(),
                        }));
                        group.clear();
                    }
                    rlist.push(inner);
                } else {
                    group.push(e.contents);
                }
            } else {
                if !group.is_empty() {
                    rlist.push(BaseElem::WordElem(WordBranch {
                        contents: group.clone(),
                    }));
                    group.clear();
                }
                rlist.push(inner);
            }
        }
        if !group.is_empty() {
            rlist.push(BaseElem::WordElem(WordBranch {
                contents: group.clone(),
            }));
            group.clear();
        }
        return Ok(rlist);
    }

    fn grouping_syntaxbox(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str>;

    ///
    /// TODO: Word以外について`()`が付与され呼ばれたときに
    /// 関数として認識できるようにする必要がある
    /// 例えば以下のような場合について
    /// ```lichen
    /// funcA()() // 関数を返却するような関数
    /// a[]()     // 関数を保持しているリスト
    /// ```
    fn grouping_functioncall<T>(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        let mut flag: bool = false;
        let mut name_tmp: Option<BaseElem> = None;
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in codelist {
            if let BaseElem::WordElem(ref wb) = inner {
                // Case WordElem
                if flag {
                    if let Some(e) = name_tmp {
                        rlist.push(e);
                    }
                }
                name_tmp = Some(inner);
                flag = true;
            } else if let BaseElem::FuncElem(ref fb) = inner {
                // Case FuncElem
                if flag {
                    if let Some(e) = name_tmp {
                        rlist.push(e);
                    }
                }
                name_tmp = Some(inner);
                flag = true;
            } else if let BaseElem::ParenBlockElem(ref pbb) = inner {
                // Case ParenBlockElem
                if flag {
                    if let Some(ref base_e) = name_tmp {
                        if let BaseElem::WordElem(ref wb) = base_e {
                            if <Self as Parser>::CONTROL_STATEMENT.contains(&(&wb.contents as &str))
                            {
                                rlist.push(BaseElem::FuncElem(FuncBranch {
                                    name: Box::new(base_e.clone()),
                                    contents: pbb.clone(),
                                    depth: self.get_depth(),
                                    loopdepth: self.get_loopdepth(),
                                }));
                                name_tmp = None;
                                flag = false;
                            } else {
                                // name tmp is not none
                                rlist.push(base_e.clone()); // contents of name_tmp -> base_e
                                rlist.push(inner);
                                name_tmp = None;
                            }
                        } else if let BaseElem::FuncElem(_) = base_e {
                            rlist.push(BaseElem::FuncElem(FuncBranch {
                                name: Box::new(base_e.clone()),
                                contents: pbb.clone(),
                                depth: self.get_depth(),
                                loopdepth: self.get_loopdepth(),
                            }));
                            name_tmp = None;
                            flag = false;
                        } else {
                            // name tmp is not none
                            rlist.push(base_e.clone()); // contents of name_tmp -> base_e
                            rlist.push(inner);
                            name_tmp = None;
                        }
                    } else {
                        //name tmp is none
                        rlist.push(inner);
                        flag = false;
                        name_tmp = None;
                    }
                }
            } else {
                // pass
            }
        }
        if flag {
            if let Some(e) = name_tmp {
                rlist.push(e);
            }
        }
        return Ok(rlist);
    }
}

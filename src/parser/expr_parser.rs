use std::vec;

use crate::abs::ast::*;
use crate::parser::core_parser::*;

use crate::token::operator::OperatorBranch;
use crate::token::syntax::SyntaxBranch;
use crate::token::syntax_box::SyntaxBoxBranch;
use crate::token::unknown::UnKnownBranch;

pub struct ExprParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ExprParser {
    /// 演算子をまとめる
    /// 演算子が長いものから順番にまとめていく必要がある
    /// 例えば、
    /// `<`より`<=`は最初にgroupingされる必要がある
    fn grouoping_operator_unit(
        &self,
        codelist: Vec<BaseElem>,
        ope: String,
    ) -> Result<Vec<BaseElem>, &str> {
        let mut group: String = String::new();
        let mut rlist: Vec<BaseElem> = Vec::new();

        let ope_size = ope.len();
        for inner in codelist {
            if let BaseElem::UnKnownElem(e) = inner {
                // 未解決の場合
                group.push(e.contents);
                if group.len() < ope_size {
                    continue;
                } else if ope_size == group.len() {
                    if group == ope {
                        rlist.push(BaseElem::OpeElem(OperatorBranch {
                            ope: group.clone(),
                            depth: self.depth,
                        }))
                    } else {
                        // rlist += group
                        let grouup_tmp: Vec<BaseElem> = group
                            .chars()
                            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                            .collect();
                        rlist.extend(grouup_tmp);
                    }
                } else {
                    // ope_size < group.len()
                    // rlist += group
                    let grouup_tmp: Vec<BaseElem> = group
                        .chars()
                        .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                        .collect();
                    rlist.extend(grouup_tmp);
                }
                group.clear();
            } else {
                // 既にtokenが割り当てられているとき
                if group.len() < ope_size {
                    // rlist += group
                    let grouup_tmp: Vec<BaseElem> = group
                        .chars()
                        .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                        .collect();
                    rlist.extend(grouup_tmp);
                } else if ope_size == group.len() {
                    if group == ope {
                        rlist.push(BaseElem::OpeElem(OperatorBranch {
                            ope: group.clone(),
                            depth: self.depth,
                        }))
                    } else {
                        // rlist += group
                        let grouup_tmp: Vec<BaseElem> = group
                            .chars()
                            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                            .collect();
                        rlist.extend(grouup_tmp);
                    }
                } else {
                    // rlist += group
                    let grouup_tmp: Vec<BaseElem> = group
                        .chars()
                        .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                        .collect();
                    rlist.extend(grouup_tmp);
                }
                group.clear();
                rlist.push(inner);
            }
        } //end of "for inner in codelist"
        return Ok(rlist);
    }

    fn grouoping_operator(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        todo!()
    }
}

impl Parser<'_> for ExprParser {
    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: code,
            depth: depth,
            loopdepth: loopdepth,
        }
    }

    fn resolve(&self) -> Result<Vec<BaseElem>, String> {
        let code_list_data = self.code2_vec_pre_proc_func(&self.code);
        let code_list = self.code2vec(&code_list_data);
        match code_list {
            Ok(mut v) => {
                for i in &mut v {
                    match i.resolve_self() {
                        Ok(_) => { /* pass */ }
                        //Err(e) => return Err(e)
                        Err(_) => { /* pass */ }
                    }
                }
                return Ok(v);
            }
            Err(e) => {
                return Err(String::from(e));
            }
        }
    }

    /// the function that groups token
    fn code2vec(&self, code: &Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        // -----    macro   -----
        /// # err_proc
        /// errorがあればErr()を返却、なければ値を返す
        macro_rules! err_proc {
            ($grouping_func:expr) => {
                match $grouping_func {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                }
            };
        }
        // ----- start code -----
        let mut code_list;
        code_list = err_proc!(self.grouping_quotation(code.to_vec()));
        code_list = err_proc!(self.grouping_elements(code_list, BaseElem::BlockElem, '{', '}'));
        code_list = err_proc!(self.grouping_elements(code_list, BaseElem::ListBlockElem, '[', ']'));
        code_list =
            err_proc!(self.grouping_elements(code_list, BaseElem::ParenBlockElem, '(', ')'));
        code_list =
            err_proc!(self.grouping_word(code_list, vec![' ', '\t', '\n'], vec![',', ';', ':']));
        return Ok(code_list);
    }

    fn get_depth(&self) -> isize {
        self.depth
    }

    fn get_loopdepth(&self) -> isize {
        self.loopdepth
    }

    fn grouping_syntaxbox(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        let mut flag = false;
        let mut name: String = String::new();
        let mut group: Vec<SyntaxBranch> = Vec::new();
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in codelist {
            if let BaseElem::SyntaxElem(ref e) = inner {
                if Self::SYNTAX_WORDS_HEADS.contains(&e.name.as_str()) {
                    flag = true;
                    name = e.name.clone();
                    group.push(e.clone());
                } else if e.name == "elif" {
                    if flag {
                        group.push(e.clone());
                    } else {
                        return Err("please write \"if\",\"while\" or \"for\" statement head");
                        // TODO:
                    }
                } else if e.name == "else" {
                    if flag {
                        group.push(e.clone());
                        rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
                            name: name.clone(),
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                        name = String::from("");
                        flag = false;
                    } else {
                        return Err("please write \"if\",\"while\" or \"for\" statement head");
                        // TODO:
                    }
                } else {
                    rlist.push(inner);
                }
            } else {
                if flag {
                    if !group.is_empty() {
                        rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
                            name: name.clone(),
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                        name = String::from("");
                    } else {
                        //pass
                    }
                    flag = false;
                } else {
                    //pass
                }
                rlist.push(inner);
            }
        }
        if !group.is_empty() {
            rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
                name: name.clone(),
                contents: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        return Ok(rlist);
    }
}

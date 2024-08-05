use crate::abs::ast::*;
use crate::parser::core_parser::*;

use crate::token::syntax::SyntaxBranch;
use crate::token::syntax_box::SyntaxBoxBranch;

pub struct ExprParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub depth: isize,
    pub loopdepth: isize,
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
        // let codelist = self.code2vec(&self.code2_vec_pre_proc_func(code));
        // for i in codelist{

        // }
        // return codelist;
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

    fn code2vec(&self, code: &Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        // macro
        macro_rules! grouping_quos {
            ($code_list:ident , $code:expr) => {
                match self.grouping_quotation($code) {
                    Ok(r) => $code_list = r,
                    Err(e) => return Err(e),
                }
            };
        }
        macro_rules! groupin_blocks {
            ($code_list:ident ,$func:path, $open_char:expr, $close_char: expr) => {
                match self.grouping_elements($code_list, $func, $open_char, $close_char) {
                    Ok(r) => $code_list = r,
                    Err(e) => return Err(e),
                }
            };
        }
        macro_rules! grouping_words {
            ($code_list:ident , $split:expr, $exclude:expr) => {
                match self.grouping_word($code_list, $split, $exclude) {
                    Ok(r) => $code_list = r,
                    Err(e) => return Err(e),
                }
            };
        }
        let mut code_list;
        grouping_quos!(code_list, code.to_vec());
        groupin_blocks!(code_list, BaseElem::BlockElem, '{', '}');
        groupin_blocks!(code_list, BaseElem::ListBlockElem, '[', ']');
        groupin_blocks!(code_list, BaseElem::ParenBlockElem, '(', ')');
        grouping_words!(code_list, vec![' ', '\t', '\n'], vec![',', ';', ':']);
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

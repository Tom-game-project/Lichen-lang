use crate::abs::ast::*;
use crate::parser::core_parser::*;
use crate::parser::parser_errors::ParserError;
use crate::token::string::StringBranch;

pub struct StateParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub code_list: Vec<BaseElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl StateParser {
    fn grouping_quotation2(&mut self) -> Result<(), ParserError> {
        let mut open_flag = false;
        let mut escape_flag = false;
        let mut rlist = Vec::new();
        let mut group = String::new();

        for inner in &self.code_list {
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
                            rlist.push(inner.clone());
                        }
                    }
                }
            } else {
                rlist.push(inner.clone());
            }
        }
        if open_flag {
            return Err(ParserError::QuotationNotClosed);
        }
        self.code_list = rlist;
        return Ok(());
    }

    fn grouping_elements2<T>(
        &mut self,
        elemtype: fn(T) -> BaseElem,
        open_char: char,
        close_char: char,
    ) -> Result<(), ParserError>
    where
        T: ASTAreaBranch,
    {
        let mut rlist: Vec<BaseElem> = Vec::new();
        let mut group: Vec<BaseElem> = Vec::new();
        let mut depth: isize = 0;

        for inner in &self.code_list {
            if let BaseElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    if depth > 0 {
                        group.push(inner.clone());
                    } else if depth == 0 {
                        // pass
                    } else {
                        return Err(ParserError::Uncategorized);
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    if depth > 0 {
                        group.push(inner.clone());
                    } else if depth == 0 {
                        rlist.push(elemtype(ASTAreaBranch::new(
                            Some(group.clone()),
                            self.get_depth(),
                            self.get_loopdepth(),
                        )));
                        group.clear();
                    } else {
                        return Err(ParserError::Uncategorized);
                    }
                } else {
                    if depth > 0 {
                        group.push(inner.clone());
                    } else if depth == 0 {
                        rlist.push(inner.clone());
                    } else {
                        return Err(ParserError::Uncategorized);
                    }
                }
            } else {
                if depth > 0 {
                    group.push(inner.clone());
                } else if depth == 0 {
                    rlist.push(inner.clone());
                } else {
                    return Err(ParserError::BraceNotClosed);
                }
            }
        }
        if depth != 0 {
            return Err(ParserError::BraceNotClosed);
        }
        self.code_list = rlist;
        return Ok(());
    }

    fn code2vec2(&mut self) -> Result<(), ParserError> {
        if let Err(e) = self.grouping_quotation2() {
            return Err(e);
        }
        if let Err(e) = self.grouping_elements2(
            BaseElem::BlockElem,
            Self::BLOCK_BRACE_OPEN,  // {
            Self::BLOCK_BRACE_CLOSE, // }
        ) {
            return Err(e);
        }
        if let Err(e) = self.grouping_elements2(
            BaseElem::ListBlockElem,
            Self::BLOCK_LIST_OPEN,  // [
            Self::BLOCK_LIST_CLOSE, // ]
        ) {
            return Err(e);
        }
        if let Err(e) = self.grouping_elements2(
            BaseElem::ParenBlockElem,
            Self::BLOCK_PAREN_OPEN,  // (
            Self::BLOCK_PAREN_CLOSE, // )
        ) {
            return Err(e);
        }
        return Ok(());
    }
}

impl Parser<'_> for StateParser {
    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: code,
            code_list: Vec::new(),
            depth: depth,
            loopdepth: loopdepth,
        }
    }

    fn resolve(&self) -> Result<Vec<BaseElem>, &str> {
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
                return Err(e);
            }
        }
    }

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
        code_list = err_proc!(self.grouping_elements(
            code_list,
            BaseElem::BlockElem,
            Self::BLOCK_BRACE_OPEN,  // {
            Self::BLOCK_BRACE_CLOSE  // }
        ));
        code_list = err_proc!(self.grouping_elements(
            code_list,
            BaseElem::ListBlockElem,
            Self::BLOCK_LIST_OPEN,  // [
            Self::BLOCK_LIST_CLOSE  // ]
        ));
        code_list = err_proc!(self.grouping_elements(
            code_list,
            BaseElem::ParenBlockElem,
            Self::BLOCK_PAREN_OPEN,  // (
            Self::BLOCK_PAREN_CLOSE  // )
        ));
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

    // grouping functions
    fn grouping_syntaxbox(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        todo!()
    }
}

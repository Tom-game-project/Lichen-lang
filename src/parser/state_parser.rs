use crate::abs::ast::*;
use crate::parser::core_parser::*;

pub struct StateParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub depth: isize,
    pub loopdepth: isize,
}

impl Parser<'_> for StateParser {
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

    fn code2vec(&self, code: &Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        let mut code_list;
        match self.grouping_quotation(code.to_vec()) {
            Ok(r) => code_list = r,
            Err(e) => return Err(e),
        }
        match self.grouping_elements(code_list, BaseElem::BlockElem, '{', '}') {
            Ok(r) => code_list = r,
            Err(e) => return Err(e),
        }
        match self.grouping_elements(code_list, BaseElem::ListBlockElem, '[', ']') {
            Ok(r) => code_list = r,
            Err(e) => return Err(e),
        }
        match self.grouping_elements(code_list, BaseElem::ParenBlockElem, '(', ')') {
            Ok(r) => code_list = r,
            Err(e) => return Err(e),
        }
        match self.grouping_word(code_list, vec![' ', '\t', '\n'], vec![',', ';', ':']) {
            Ok(r) => code_list = r,
            Err(e) => return Err(e),
        }
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

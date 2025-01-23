use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;

// type match
// Result<Ok, Err> : structure
// [i32; 10]       : array
// (i32, f32, f64) : tuple

#[derive(Clone, Debug)]
pub struct TypeBlockBranch {
    pub code_list: Vec<TypeElem>,
    pub depth: isize,
    // loopdepth: isize,
}

impl TypeBlockBranch {}

impl ASTBranch for TypeBlockBranch {
    fn get_show_as_string(&self) -> String {
        let mut rstr = String::new();
        for i in &self.code_list {
            rstr = format!("{}{}", rstr, i.get_show_as_string());
        }
        format!("<{}>", rstr)
    }

    fn show(&self) {
        println!("{}", self.get_show_as_string())
    }
}

impl TypeAreaBranch for TypeBlockBranch {
    fn new(code_list: Vec<TypeElem>, depth: isize) -> Self {
        Self { code_list, depth }
    }
}

impl RecursiveAnalysisElements for TypeBlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        // TODO!
        Ok(())
    }
}


impl RecursiveAnalysisTypeElements for TypeBlockBranch {
    fn resolve_self_as_type(&mut self) -> Result<(), ParserError> {
        // TODO
        Ok(())
    }
}

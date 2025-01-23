
use crate::abs::ast::*;
use crate::parser::type_parser::TypeParser;
use crate::errors::parser_errors::ParserError;

#[derive(Clone, Debug)]
pub struct TypeFuncBranch {
    pub name: String,
    pub contents: Vec<TypeElem>,
    pub depth:isize,
    pub loopdepth:isize,
}

impl ASTBranch for TypeFuncBranch {
    fn show(&self) {
        println!("{}", self.get_show_as_string());
    }

    fn get_show_as_string(&self) -> String {
        let mut inner_string = String::default();
        for i in &self.contents {
            inner_string.push_str(&i.get_show_as_string());
        }
        format!("{}({})", self.name, inner_string)
    }
}

impl RecursiveAnalysisTypeElements for TypeFuncBranch {
    fn resolve_self_as_type(&mut self) -> Result<(), ParserError> {
        //for i in &mut self.contents{
        //    i.resolve_self()?;
        //}
        //Ok(())

        let mut parser = 
            TypeParser::create_parser_from_vec(self.contents.clone(), 0, 0);
        parser.code2vec()?;
        let mut rlist = parser.code_list;
        for i in &mut rlist{
            i.resolve_self()?;
        }
        self.contents = rlist;
        Ok(())
    }
}


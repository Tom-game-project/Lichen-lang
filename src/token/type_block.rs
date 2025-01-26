
use crate::abs::ast::*;
use crate::parser::type_parser::TypeParser;
use crate::errors::parser_errors::ParserError;

#[derive(Clone, Debug)]
pub struct TypeBlockBranch {
    pub name: String,
    pub contents: Vec<TypeElem>,
    pub depth:isize,
    pub loopdepth:isize,
}

impl ASTBranch for TypeBlockBranch {
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

impl RecursiveAnalysisTypeElements for TypeBlockBranch {
    fn resolve_self_as_type(&mut self) -> Result<(), ParserError> {
        let mut parser = 
            TypeParser::create_parser_from_vec(self.contents.clone(), 0, 0);
        parser.code2vec()?;
        parser.comma_parser()?;
        let mut rlist = parser.code_list;
        for i in &mut rlist{
            i.resolve_self()?;
        }
        self.contents = rlist;
        Ok(())
    }
}


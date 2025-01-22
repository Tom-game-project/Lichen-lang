use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::type_parser::TypeParser;

/// 引数などの式を格納します
#[derive(Clone, Debug)]
pub struct TypeItemBranch {
    pub contents: Vec<TypeElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisTypeElements for TypeItemBranch{
    fn resolve_self_as_type(&mut self) -> Result<(), ParserError> {
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

impl ASTBranch for TypeItemBranch {
    fn show(&self) {
        println!("{}Item(", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            i.show();
        }
        println!("{})", " ".repeat(self.depth as usize * 4));

    }

    fn get_show_as_string(&self) -> String {
        let mut rstr = format!("{}Item(\n", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            rstr = format!("{}{}", rstr, i.get_show_as_string());
        }
        rstr = format!("{}\n{})", rstr, " ".repeat(self.depth as usize * 4));
        rstr

    }
}


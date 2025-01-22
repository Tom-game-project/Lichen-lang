use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;
use crate::parser::type_parser::{expr2type, TypeParser};

/// #ParenBlockBranch
/// `()`を使用したプログラムにおけるデータを格納するstruct
/// 中では,
/// - 式を解析する必要がある場合
/// - タイプ宣言を解析する必要がある場合２ ex) (T, T)
#[derive(Clone, Debug)]
pub struct ParenBlockBranch {
    pub contents: Vec<ExprElem>,
    pub contents_as_type: Vec<TypeElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for ParenBlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        // 式パーサーによって解析
        let mut parser = ExprParser::create_parser_from_vec(
            self.contents.clone(),
            self.depth + 1,
            self.loopdepth,
        );
        match parser.code2vec() {
            Ok(_) => {
                let mut rlist = parser.code_list;
                for i in &mut rlist {
                    i.resolve_self()?
                }
                self.contents = rlist;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl RecursiveAnalysisTypeElements for ParenBlockBranch {
    fn resolve_self_as_type(&mut self) -> Result<(), ParserError> {
        // 型パーサによって解析
        // parenのなかでカンマ区切りで、宣言されるかもしれない
        // ```
        // (i32)
        // (i32, i32)
        // (<type>, <type>, ...)
        // (a: i32, b: i32) // ex
        // ```
        // exは
        // ```
        // (a: <type>, b: <type>)
        // ```
        // と解釈すれば解決できる
        // その上で、ここには、カンマで木々って解釈しなければならない
        let mut parser = TypeParser::create_parser_from_vec(
            self.contents_as_type.clone(),
            0, 0
        );
        // println!("in_resolve_self_as_type function{:?}", parser.code_list); //すでに消えてる
        parser.code2vec()?;
        // ここで、カンマごとに区切るcode2vecとは別の関数を用意する
        parser.comma_parser()?;
        let mut rlist = parser.code_list;
        for i in &mut rlist {
            i.resolve_self()?; // 呼び出した先でresolve_self_as_typeが更に呼ばれる
        }
        self.contents_as_type = rlist;
        Ok(())
    }
}

impl ASTBranch for ParenBlockBranch {
    fn show(&self) {
        println!("{}Paren\n(", " ".repeat(self.depth as usize));
        for i in &self.contents {
            i.show();
        }
        println!("{})", " ".repeat(self.depth as usize));
    }

    fn get_show_as_string(&self) -> String {
        let open_section = format!("{}Paren\n(", " ".repeat(self.depth as usize));
        let mut group_section = String::new();
        for i in &self.contents {
            group_section = format!("{}{}", group_section, i.get_show_as_string());
        }
        let close_section = format!("{})", " ".repeat(self.depth as usize));
        format!("{}{}{}", open_section, group_section, close_section)
    }
}

impl ASTAreaBranch<ExprElem> for ParenBlockBranch {
    fn new(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents,
            contents_as_type: Vec::default(),
            depth,
            loopdepth,
        }
    }
}

impl TypeAreaBranch for ParenBlockBranch{
    fn new(contents: Vec<TypeElem>, depth: isize) -> Self {
        // println!("contents you want to insert {:?}", contents);
        Self {
            contents: Vec::default(),
            contents_as_type: contents, 
            depth,
            loopdepth: 0 
        }
    }
}

use crate::parser::core_parser::*;
use crate::errors::parser_errors::ParserError;

use crate::abs::ast::*;
use crate::token::word::WordBranch;
use crate::token::type_item::TypeItemBranch;

/// まとめられるケース
/// `i32`, `i64`, `f32`, `f64`, `+<+>`

pub struct TypeParser {
    pub code: String,
    pub code_list: Vec<TypeElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl TypeParser {
    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_elements(TypeElem::TypeBlockElem, Self::BLOCK_TYPE_OPEN,Self::BLOCK_TYPE_CLOSE)?;
        self.grouping_words()?;
        Ok(())
    }

    /// code2vecとは別に、カンマ区切りに分割する関数を作る
    /// この関数は、`code2vec`関数のあとに実行する必要があります。
    pub fn  comma_parser(&mut self) -> Result<(), ParserError>
    {
        let mut rlist:Vec<TypeElem> = Vec::new();
        let mut group:Vec<TypeElem> = Vec::new();

        for inner in &self.code_list {
            if let TypeElem::UnKnownElem(ub) = inner {
                if ub.contents == '\'' // TODO: magic number
                {
                    // itemをrlistに追加
                    rlist.push(
                        TypeElem::ItemBlockElem(
                            TypeItemBranch {
                                contents: group.clone(),
                                depth:self.depth,
                                loopdepth:self.loopdepth
                            }
                    ));
                    group.clear();
                }
                else
                {
                    group.push(inner.clone());
                }
            }
            else 
            {
                //
                group.push(inner.clone());
            }
        }
        if !group.is_empty()
        {
            // itemをrlistに追加
            rlist.push(
                TypeElem::ItemBlockElem(
                    TypeItemBranch {
                        contents: group.clone(),
                        depth:self.depth,
                        loopdepth:self.loopdepth
                    }
            ));
            group.clear();
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_elements<T>(
        &mut self,
        elemtype: fn(T) -> TypeElem,
        open_char: char,
        close_char: char,
    ) -> Result<(), ParserError>
    where
        T: TypeAreaBranch,
    {
        let mut rlist: Vec<TypeElem> = Vec::new();
        let mut group: Vec<TypeElem> = Vec::new();
        let mut depth: isize = 0;

        for inner in &self.code_list {
            if let TypeElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    match depth {
                        0 => { /*pass*/ }
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    match depth {
                        0 => {
                            rlist.push(elemtype(TypeAreaBranch::new(group.clone(), self.depth)));
                            group.clear();
                        }
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                } else {
                    match depth {
                        0 => rlist.push(inner.clone()),
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                }
            } else {
                match depth {
                    0 => rlist.push(inner.clone()),
                    1.. => group.push(inner.clone()),
                    _ => return Err(ParserError::BraceNotClosed),
                }
            }
        }
        if depth != 0 {
            return Err(ParserError::BraceNotClosed);
        }
        self.code_list = rlist;
        Ok(())
    }
    
    fn grouping_words(&mut self) -> Result<(), ParserError>
    {
        // macro
        let mut rlist: Vec<TypeElem> = Vec::new();
        let mut group: String = String::new();

        for inner in &self.code_list {
            if let TypeElem::UnKnownElem(ref e) = inner {
                if Self::SPLIT_CHAR.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        rlist.push(
                            TypeElem::WordElem(
                                WordBranch{
                                    contents: group.clone(),
                                    depth: self.depth,
                                    loopdepth: self.loopdepth
                                }
                            )
                        );
                        group.clear();
                    }
                } 
                else {
                    group.push(e.contents);
                }
            } else {
                if !group.is_empty() {
                    rlist.push(
                        TypeElem::WordElem(
                            WordBranch{
                                contents: group.clone(),
                                depth: self.depth,
                                loopdepth: self.loopdepth
                            }
                        )
                    );
                    group.clear();
                }
                rlist.push(inner.clone());
            }
        }
        if !group.is_empty() {
            rlist.push(
                TypeElem::WordElem(
                    WordBranch{
                        contents: group.clone(),
                        depth: self.depth,
                        loopdepth: self.loopdepth
                    }
                )
            );
            group.clear();
        }
        self.code_list = rlist;
        Ok(())
    }


    /// ExprElemの途中までパースされた集合をtype用に変換する
    pub fn create_parser_from_vec(
        code_list: Vec<TypeElem>,
        depth: isize,
        loopdepth: isize,
    ) -> Self {
        Self {
            code: String::new(),
            code_list,
            depth,
            loopdepth,
        }
    }
}

pub fn expr2type(i: &[ExprElem]) -> Result<Vec<TypeElem>, ParserError> {
    let mut rlist: Vec<TypeElem> = Vec::new();

    for inner in i.iter(){
        rlist.push(match inner {
            ExprElem::CommentElem(cb) => {
                TypeElem::CommentElem(cb.clone())
            }
            ExprElem::WordElem(wb) => {
                TypeElem::WordElem(wb.clone())
            }
            ExprElem::ParenBlockElem(pb) => {
                TypeElem::ParenBlockElem(pb.clone())
            }
            ExprElem::ListBlockElem(lb) => {
                TypeElem::ListBlockElem(lb.clone())
            }
            ExprElem::UnKnownElem(ub) => {
                TypeElem::UnKnownElem(ub.clone())
            }
            _ => {
                println!("while convert expr 2 type");
                return Err(ParserError::UnableToConvertType);
            }
        });
    }
    Ok(rlist)
}

impl Parser<'_> for TypeParser
{
    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: code.clone(),
            code_list: Self::code2_vec_pre_proc_func(&code),
            depth,
            loopdepth
        }
    }

    fn resolve(&mut self) -> Result<(), ParserError> {
        self.code2vec()?;
        for i in &mut self.code_list {
            i.resolve_self()?;
        }
        Ok(())
    }
}

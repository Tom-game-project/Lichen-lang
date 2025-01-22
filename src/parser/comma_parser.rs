use crate::abs::ast::ASTAreaBranch;
use crate::abs::ast::ExprElem;

use crate::abs::ast::ProcToken;
use crate::abs::ast::Token;
use crate::abs::ast::TypeElem;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;

use crate::token::item::ItemBranch;
use crate::token::string::StringBranch;

/// # CommaParser
/// This parser is a bit anomalous.
/// It performs specialized parsing of the argument part of a function that takes multiple arguments.
pub struct CommaParser {
    pub code: String,
    pub code_list: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl CommaParser {
    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_quotation()?;
        // grouping_elements
        self.grouping_elements(
            ExprElem::BlockElem,
            Self::BLOCK_BRACE_OPEN,  // {
            Self::BLOCK_BRACE_CLOSE, // }
        )?;
        self.grouping_elements(
            ExprElem::ListBlockElem,
            Self::BLOCK_LIST_OPEN,  // [
            Self::BLOCK_LIST_CLOSE, // ]
        )?;
        self.grouping_elements(
            ExprElem::ParenBlockElem,
            Self::BLOCK_PAREN_OPEN,  // (
            Self::BLOCK_PAREN_CLOSE, // )
        )?;
        self.grouping_args()?;
        Ok(())
    }

    fn grouping_args(&mut self) -> Result<(), ParserError> {
        let mut group: Vec<ExprElem> = vec![];
        let mut rlist: Vec<ExprElem> = Vec::new();

        for inner in &self.code_list {
            if let ExprElem::UnKnownElem(v) = inner {
                if v.contents == Self::COMMA {
                    rlist.push(ExprElem::ItemElem(ItemBranch {
                        contents: group.clone(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                    group.clear();
                } else {
                    group.push(inner.clone());
                }
            } else {
                group.push(inner.clone());
            }
        }
        if !group.is_empty() {
            rlist.push(ExprElem::ItemElem(ItemBranch {
                contents: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_quotation(&mut self) -> Result<(), ParserError> {
        let mut open_flag = false;
        let mut escape_flag = false;
        let mut rlist = Vec::new();
        let mut group = String::new();

        for inner in &self.code_list {
            if let ExprElem::UnKnownElem(ref v) = inner {
                if escape_flag {
                    group.push(v.contents);
                    escape_flag = false
                } else if v.contents == Self::DOUBLE_QUOTATION
                // '"'
                // is quochar
                {
                    if open_flag {
                        group.push(v.contents);
                        rlist.push(ExprElem::StringElem(StringBranch {
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                        open_flag = false;
                    } else {
                        group.push(v.contents);
                        open_flag = true;
                    }
                } else if open_flag {
                    escape_flag = v.contents == Self::ESCAPECHAR; // '\\'
                    group.push(v.contents);
                } else {
                    rlist.push(inner.clone());
                }
            } else {
                rlist.push(inner.clone());
            }
        }
        if open_flag {
            return Err(ParserError::QuotationNotClosed);
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_elements<T, U>(
        &mut self,
        elemtype: fn(T) -> ExprElem,
        open_char: char,
        close_char: char,
    ) -> Result<(), ParserError>
    where
        T: ASTAreaBranch<U>,
        U: Clone + Token + ProcToken,
    {
        let mut rlist: Vec<ExprElem> = Vec::new();
        let mut group: Vec<U> = Vec::new();
        let mut depth: isize = 0;

        for inner in &self.code_list {
            if let ExprElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    match depth {
                        0 => { /*pass*/ }
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::Uncategorized),
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    match depth {
                        0 => {
                            rlist.push(elemtype(ASTAreaBranch::new(
                                group.clone(),
                                self.depth,
                                self.loopdepth,
                            )));
                            group.clear();
                        }
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::Uncategorized),
                    }
                } else {
                    match depth {
                        0 => rlist.push(inner.clone()),
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::Uncategorized),
                    }
                }
            } else {
                match depth {
                    0 => rlist.push(inner.clone()),
                    1.. => {
                        match &inner {
                            ExprElem::StringElem(s) => {
                                group.push(ProcToken::t_string(
                                    s.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            ExprElem::BlockElem(bl) => {
                                group.push(ProcToken::t_block(
                                    bl.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            ExprElem::ParenBlockElem(pb) => {
                                group.push(ProcToken::t_parenblock(
                                    pb.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            ExprElem::ListBlockElem(lb) => {
                                group.push(ProcToken::t_listblock(
                                    lb.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            // todo
                            _ => {
                                // todo error処理
                                return Err(ParserError::UnexpectedTypeComma);
                            }
                        }
                    }
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

    pub fn create_parser_from_vec(
        code_list: Vec<ExprElem>,
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

impl Parser<'_> for CommaParser {
    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code,
            code_list: Vec::new(),
            depth,
            loopdepth,
        }
    }

    fn resolve(&mut self) -> Result<(), ParserError> {
        self.code2vec()?;
        Ok(())
    }
}

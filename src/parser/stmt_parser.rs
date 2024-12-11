use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::*;

use crate::token::comment::CommentBranch;
use crate::token::decfunc::DecFuncBranch;
use crate::token::operator::OperatorBranch;
use crate::token::stmt::expr::ExprBranch;
use crate::token::stmt::stmt::StmtBranch;
use crate::token::string::StringBranch;
use crate::token::word::WordBranch;

/// # StmtParser
pub struct StmtParser {
    pub code: String,
    pub code_list: Vec<StmtElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

enum StringAreaState {
    CommentOpen,  // /*
    CommentStart, // //
    QuotationOpen,
    Closed,
}

impl StmtParser {
    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_string()?;
        self.grouping_elements(
            StmtElem::BlockElem,
            Self::BLOCK_BRACE_OPEN,  // {
            Self::BLOCK_BRACE_CLOSE, // }
        )?;
        self.grouping_elements(
            StmtElem::ListBlockElem,
            Self::BLOCK_LIST_OPEN,  // [
            Self::BLOCK_LIST_CLOSE, // ]
        )?;
        self.grouping_elements(
            StmtElem::ParenBlockElem,
            Self::BLOCK_PAREN_OPEN,  // (
            Self::BLOCK_PAREN_CLOSE, // )
        )?;
        self.grouping_words()?;
        // ここで関数の宣言処理を加える
        self.grouping_function_definition()?;
        self.split_semicolon()?;
        Ok(())
    }

    fn grouping_string(&mut self) -> Result<(), ParserError> {
        // now this function can group all string in  the program
        let mut group: String = String::new();
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut open_status: StringAreaState = StringAreaState::Closed;
        let mut ignore_flag = false;
        let mut string_escape_flag = false;

        for (count, inner) in self.code_list.iter().enumerate() {
            if ignore_flag {
                ignore_flag = false;
                // 二文字の判別
                continue;
            }
            if let StmtElem::UnKnownElem(e) = inner {
                match open_status {
                    StringAreaState::CommentStart => {
                        // //が開いているとき
                        if e.contents == '\n' {
                            rlist.push(StmtElem::CommentElem(CommentBranch {
                                contents: group.clone(),
                                depth: self.depth,
                                loopdepth: self.loopdepth,
                            }));
                            open_status = StringAreaState::Closed;
                            group.clear();
                        } else {
                            group.push(e.contents);
                        }
                    }
                    StringAreaState::CommentOpen => {
                        // /*が開いているとき
                        if Self::COMMENT_CLOSE.starts_with(e.contents)
                        // "*" == e.content
                        {
                            if count < self.code_list.len() {
                                if let StmtElem::UnKnownElem(next_e) = &self.code_list[count + 1] {
                                    if Self::COMMENT_CLOSE.ends_with(next_e.contents)
                                    // "/" == e.content
                                    {
                                        rlist.push(StmtElem::CommentElem(CommentBranch {
                                            contents: group.clone(),
                                            depth: self.depth,
                                            loopdepth: self.loopdepth,
                                        }));
                                        group.clear();
                                        open_status = StringAreaState::Closed;
                                        ignore_flag = true;
                                    } else {
                                        group.push(e.contents);
                                    }
                                } else {
                                    // defer type
                                    return Err(ParserError::UnexpectedTypeStmt);
                                }
                            } else {
                                return Err(ParserError::CommentBlockNotClosed);
                            }
                        } else {
                            group.push(e.contents);
                        }
                    }
                    StringAreaState::QuotationOpen => {
                        // '"' is opened
                        if Self::DOUBLE_QUOTATION == e.contents {
                            if string_escape_flag {
                                group.push(e.contents);
                                string_escape_flag = false;
                            } else {
                                rlist.push(StmtElem::StringElem(StringBranch {
                                    contents: group.clone(),
                                    depth: self.depth,
                                    loopdepth: self.loopdepth,
                                }));
                                group.clear();
                                open_status = StringAreaState::Closed;
                            }
                        } else if Self::ESCAPECHAR == e.contents {
                            if string_escape_flag {
                                group.push(e.contents);
                                string_escape_flag = false;
                            } else {
                                string_escape_flag = true;
                            }
                        } else {
                            group.push(e.contents);
                        }
                    }
                    StringAreaState::Closed => {
                        // 何も開いていないとき
                        if Self::COMMENT_OPEN.starts_with(e.contents)
                            || Self::COMMENT_START.starts_with(e.contents)
                        {
                            if count < self.code_list.len() {
                                if let StmtElem::UnKnownElem(next_e) = &self.code_list[count + 1] {
                                    if Self::COMMENT_OPEN.ends_with(next_e.contents) {
                                        open_status = StringAreaState::CommentOpen;
                                        ignore_flag = true;
                                    } else if Self::COMMENT_START.ends_with(next_e.contents) {
                                        open_status = StringAreaState::CommentStart;
                                        ignore_flag = true;
                                    } else {
                                        rlist.push(inner.clone())
                                    }
                                } else {
                                    rlist.push(inner.clone());
                                }
                            } else {
                                rlist.push(inner.clone());
                            }
                        } else if Self::DOUBLE_QUOTATION == e.contents {
                            open_status = StringAreaState::QuotationOpen;
                        } else {
                            rlist.push(inner.clone());
                        }
                    }
                }
            } else {
                rlist.push(inner.clone());
            }
        }
        if let StringAreaState::CommentStart = open_status {
            rlist.push(StmtElem::CommentElem(CommentBranch {
                contents: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        } else if let StringAreaState::CommentOpen = open_status {
            return Err(ParserError::CommentBlockNotClosed);
        } else if let StringAreaState::QuotationOpen = open_status {
            return Err(ParserError::QuotationNotClosed);
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_elements<T, U>(
        &mut self,
        elemtype: fn(T) -> StmtElem,
        open_char: char,
        close_char: char,
    ) -> Result<(), ParserError>
    where
        T: ASTAreaBranch<U>,
        U: Clone + Token + ProcToken, // ExprElem or StmtElem
    {
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: Vec<U> = Vec::new();
        let mut depth: isize = 0;

        for inner in &self.code_list {
            if let StmtElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    match depth {
                        0 => { /*pass*/ }
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    match depth {
                        0 => {
                            rlist.push(elemtype(ASTAreaBranch::<U>::new(
                                group.clone(),
                                self.depth,
                                self.loopdepth,
                            )));
                            group.clear();
                        }
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                } else {
                    match depth {
                        0 => rlist.push(inner.clone()),
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                }
            } else {
                match depth {
                    0 => rlist.push(inner.clone()),
                    1.. => {
                        match &inner {
                            StmtElem::StringElem(s) => {
                                group.push(ProcToken::t_string(
                                    s.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            StmtElem::BlockElem(bl) => {
                                group.push(ProcToken::t_block(
                                    bl.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            StmtElem::ParenBlockElem(pb) => {
                                group.push(ProcToken::t_parenblock(
                                    pb.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            StmtElem::ListBlockElem(lb) => {
                                group.push(ProcToken::t_listblock(
                                    lb.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            StmtElem::CommentElem(cb) => {
                                group.push(ProcToken::t_commentblock(
                                    cb.contents.clone(),
                                    cb.depth,
                                    cb.loopdepth,
                                ));
                            }
                            // todo
                            _ => {
                                // todo error処理
                                println!("some thing wrong");
                                println!("{:?}", inner);
                                return Err(ParserError::UnexpectedTypeStmt);
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

    fn grouping_words(&mut self) -> Result<(), ParserError> {
        // macro
        macro_rules! add_rlist {
            ($rlist:expr,$group:expr) => {
                if let Ok(_) = Self::find_ope_priority(&$group) {
                    $rlist.push(StmtElem::OpeElem(OperatorBranch {
                        ope: $group.clone(),
                        depth: self.depth,
                    }))
                } else {
                    $rlist.push(StmtElem::WordElem(WordBranch {
                        contents: $group.clone(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                }
            };
        }
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: String = String::new();
        let ope_str = Self::LENGTH_ORDER_OPE_LIST.map(|a| a.opestr).join("");

        for inner in &self.code_list {
            if let StmtElem::UnKnownElem(ref e) = inner {
                if Self::SPLIT_CHAR.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        add_rlist!(rlist, group);
                        group.clear();
                    }
                } else if Self::EXCLUDE_WORDS.contains(&e.contents) || ope_str.contains(e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        add_rlist!(rlist, group);
                        group.clear();
                    }
                    rlist.push(inner.clone());
                } else {
                    group.push(e.contents);
                }
            } else {
                if !group.is_empty() {
                    add_rlist!(rlist, group);
                    group.clear();
                }
                rlist.push(inner.clone());
            }
        }
        if !group.is_empty() {
            add_rlist!(rlist, group);
            group.clear();
        }
        self.code_list = rlist;
        Ok(())
    }

    /// 関数の宣言をまとめる
    ///
    pub fn grouping_function_definition(&mut self) -> Result<(), ParserError> {
        struct CheckList {
            start_flag:bool,
            func_name_flag:bool,
            paren_flag:bool,
            type_flag:bool,
        }
        let mut rlist :Vec<StmtElem> = Vec::default();
        let mut group :Vec<StmtElem> = Vec::default();
        let mut check_list = CheckList{ start_flag:false,
            func_name_flag:false,
            paren_flag:false,
            type_flag:false,
        };
        let mut func_name = String::default();
        let mut paren_list :Vec<ExprElem>= Vec::default();
        let mut type_list :Vec<StmtElem> = Vec::default(); 

        for inner in &self.code_list {
            if check_list.start_flag && 
                !check_list.func_name_flag
            {
                if let StmtElem::WordElem(word_b) = &inner {
                    // 関数名を取得
                    func_name = word_b.contents.clone();
                    check_list.func_name_flag = true;
                } else {
                    // error
                    return Err(ParserError::InvalidFuncSyntax);
                }
            } else if check_list.start_flag && 
                 check_list.func_name_flag && 
                !check_list.paren_flag 
            {
                // 引数のparen branch
                if let StmtElem::ParenBlockElem(paren_b) = &inner 
                {
                    // 引数の内容を取得
                    paren_list = paren_b.contents.clone();
                    check_list.paren_flag = true;
                } else {
                    return Err(ParserError::InvalidFuncSyntax);
                }
            } else if check_list.start_flag && 
                check_list.func_name_flag && 
                check_list.paren_flag && 
                !check_list.type_flag 
            {
                    if let StmtElem::BlockElem(block_b) = &inner {
                        // ここで、関数宣言のまとまりが作成される
                        check_list.type_flag = true;
                        rlist.push(StmtElem::DefineElem(DecFuncBranch {
                            func_name:func_name.clone(),
                            arg_types: expr2type(&paren_list)?, // タイプとして処理する
                            return_types: stmt2type(&type_list)?, // タイプとして処理する
                            contents: block_b.clone(),
                            depth: self.depth, loopdepth: self.loopdepth }));
                        check_list = CheckList{ start_flag:false,
                            func_name_flag:false,
                            paren_flag:false,
                            type_flag:false,
                        };
                        type_list.clear();
                    } else {
                        // blockを見つけるまでは、タイプとして解釈する
                        // typeとして追加していく
                        type_list.push(inner.clone());
                    }
            } else {
                // フラグがたっていない場合
                if let StmtElem::WordElem(word_b) = &inner {
                    if word_b.contents == Self::FUNCTION {
                        // fn 
                        check_list.start_flag = true;
                    } else if word_b.contents == Self::PUB_FUNCTION {
                        // pub_fn
                        check_list.start_flag = true;
                    } else {
                        rlist.push(inner.clone());
                    }
                } else {
                    rlist.push(inner.clone());
                }
            }
        }
        self.code_list = rlist;
        Ok(())
    }

    /// function for splitting semicolon
    ///
    /// ```text
    /// // こめんと
    /// let a = 1; // <- stmt
    /// let b = 2; // <- stmt
    /// let c = 3; // <- stmt
    /// return a; // <- stmt
    /// ```
    pub fn split_semicolon(&mut self) -> Result<(), ParserError> {
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: Vec<StmtElem> = Vec::new();

        for inner in &self.code_list {
            match &inner {
                StmtElem::UnKnownElem(unb) => {
                    if unb.contents == Self::SEMICOLON {
                        if !group.is_empty() {
                            if let StmtElem::WordElem(word_b) = &group[0] {
                                if Self::CONTROL_STATEMENT.contains(&word_b.contents.as_str()) {
                                    // return 等の
                                    // 予約語だった場合
                                    rlist.push(StmtElem::Special(StmtBranch {
                                        head: word_b.contents.clone(),
                                        code_list: stmt2expr(&group[1..])?,
                                        depth: self.depth,
                                        loopdepth: self.loopdepth,
                                    }));
                                } else {
                                    // 普通の変数のwordだった場合
                                    rlist.push(StmtElem::ExprElem(ExprBranch {
                                        code_list: stmt2expr(&group)?,
                                        depth: self.depth,
                                        loopdepth: self.loopdepth,
                                    }));
                                }
                            } else {
                                // 最初の要素がwordではなかった場合
                                rlist.push(StmtElem::ExprElem(ExprBranch {
                                    code_list: stmt2expr(&group)?,
                                    depth: self.depth,
                                    loopdepth: self.loopdepth,
                                }));
                            }
                        } else {
                            // group が空だった場合
                        }
                        group.clear();
                    } else {
                        // セミコロン以外でまだ決定していないchar
                        group.push(inner.clone());
                    }
                }
                StmtElem::CommentElem(comment_b) => {
                    // コメントが文の途中で現れたとき
                    if !group.is_empty() {
                        if let StmtElem::WordElem(word_b) = &group[0] {
                            if Self::CONTROL_STATEMENT.contains(&word_b.contents.as_str()) {
                                // return 等の
                                // 予約語だった場合
                                rlist.push(StmtElem::Special(StmtBranch {
                                    head: word_b.contents.clone(),
                                    code_list: stmt2expr(&group[1..])?,
                                    depth: self.depth,
                                    loopdepth: self.loopdepth,
                                }));
                            } else {
                                // 普通の変数のwordだった場合
                                rlist.push(StmtElem::ExprElem(ExprBranch {
                                    code_list: stmt2expr(&group)?,
                                    depth: self.depth,
                                    loopdepth: self.loopdepth,
                                }));
                            }
                        } else {
                            // 最初の要素がwordではなかった場合
                            rlist.push(StmtElem::ExprElem(ExprBranch {
                                code_list: stmt2expr(&group)?,
                                depth: self.depth,
                                loopdepth: self.loopdepth,
                            }));
                        }
                    } else {
                        // group が空だった場合
                    }
                    group.clear();
                    rlist.push(StmtElem::CommentElem(CommentBranch {
                        contents: comment_b.contents.clone(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                }
                _ => {
                    group.push(inner.clone());
                }
            }
        }
        if !group.is_empty() {
            rlist.push(StmtElem::ExprElem(ExprBranch {
                code_list: stmt2expr(&group)?,
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        self.code_list = rlist;
        Ok(())
    }

    pub fn create_parser_from_vec(
        code_list: Vec<StmtElem>,
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


/// function for converting `stmt` to `expr`
fn stmt2expr(i: &[StmtElem]) -> Result<Vec<ExprElem>, ParserError> {
    let mut rlist: Vec<ExprElem> = Vec::new();
    for inner in i.iter() {
        rlist.push(match inner {
            StmtElem::StringElem(a) => ExprElem::StringElem(a.clone()),
            StmtElem::CommentElem(a) => ExprElem::CommentElem(a.clone()),
            StmtElem::BlockElem(a) => ExprElem::BlockElem(a.clone()),
            StmtElem::ListBlockElem(a) => ExprElem::ListBlockElem(a.clone()),
            StmtElem::ParenBlockElem(a) => ExprElem::ParenBlockElem(a.clone()),
            StmtElem::OpeElem(a) => ExprElem::OpeElem(a.clone()),
            StmtElem::WordElem(a) => ExprElem::WordElem(a.clone()),
            StmtElem::UnKnownElem(a) => ExprElem::UnKnownElem(a.clone()),
            _ => {
                println!("stmt2expr {:?}", inner);
                return Err(ParserError::UnableToConvertType);
            }
        });
    }
    Ok(rlist)
}

/// function for converting `expr` to `type`
fn expr2type(i: &[ExprElem]) -> Result<Vec<TypeElem>, ParserError> {
    let mut rlist: Vec<TypeElem> = Vec::new();
    for inner in i.iter() {
        rlist.push(match inner {
            ExprElem::CommentElem(a) => TypeElem::CommentElem(a.clone()),
            ExprElem::WordElem(a) => TypeElem::WordElem(a.clone()),
            ExprElem::ParenBlockElem(a) => TypeElem::ParenBlockElem(a.clone()),
            ExprElem::ListBlockElem(a) => TypeElem::ListBlockElem(a.clone()),
            ExprElem::UnKnownElem(a) => TypeElem::UnKnownElem(a.clone()),
            _ => {
                println!("expr2type {:?}", inner);
                return Err(ParserError::UnableToConvertType);
            }
        });
    }
    Ok(rlist)
}

/// function for converting `stmt` to `type`
fn stmt2type(i: &[StmtElem]) -> Result<Vec<TypeElem>, ParserError> {
    let mut rlist: Vec<TypeElem> = Vec::new();
    for inner in i.iter() {
        rlist.push(match inner {
            StmtElem::CommentElem(a) => TypeElem::CommentElem(a.clone()),
            StmtElem::WordElem(a) => TypeElem::WordElem(a.clone()),
            StmtElem::ParenBlockElem(a) => TypeElem::ParenBlockElem(a.clone()),
            StmtElem::ListBlockElem(a) => TypeElem::ListBlockElem(a.clone()),
            StmtElem::UnKnownElem(a) => TypeElem::UnKnownElem(a.clone()),
            _ => {
                println!("stmt2type {:?}", inner);
                return Err(ParserError::UnableToConvertType);
            }
        });
    }
    Ok(rlist)
}

impl Parser<'_> for StmtParser {
    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: code.clone(),
            code_list: Self::code2_vec_pre_proc_func(&code),
            depth,
            loopdepth,
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

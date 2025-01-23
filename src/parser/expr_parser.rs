use std::cmp::Ordering;

use crate::abs::ast::*;
use crate::parser::core_parser::*;

use crate::errors::parser_errors::ParserError;

use crate::token::comment::CommentBranch;
use crate::token::func::FuncBranch;
use crate::token::item::ItemBranch;
use crate::token::list::ListBranch;
use crate::token::operator::OperatorBranch;
use crate::token::paren_block::ParenBlockBranch;
use crate::token::string::StringBranch;
use crate::token::syntax::SyntaxBranch;
use crate::token::syntax_box::SyntaxBoxBranch;
use crate::token::word::WordBranch;

/// # ExprParser
///
pub struct ExprParser {
    pub code: String,
    pub code_list: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

enum StringAreaState {
    CommentOpen,  // /*
    CommentStart, // //
    QuotationOpen,
    Closed,
}

impl ExprParser {
    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_string()?;
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
        // end of grouping_elements
        self.grouping_words()?;

        // grouping syntax
        self.grouping_syntax()?;
        self.grouping_syntaxbox()?;

        while self.contain_subscriptable() {
            self.grouping_subscription()?;
        }
        self.grouping_operator()?;
        self.resolve_operation()?;
        Ok(())
    }

    fn grouping_words(&mut self) -> Result<(), ParserError> {
        // macro
        macro_rules! add_rlist {
            ($rlist:expr,$group:expr) => {
                $rlist.push(ExprElem::WordElem(WordBranch {
                    contents: $group.clone(),
                    depth: self.depth,
                    loopdepth: self.loopdepth,
                }));
            };
        }
        let mut rlist: Vec<ExprElem> = Vec::new();
        let mut group: String = String::new();
        let ope_str = Self::LENGTH_ORDER_OPE_LIST.map(|a| a.opestr).join("");

        for inner in &self.code_list {
            if let ExprElem::UnKnownElem(ref e) = inner {
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

    fn grouping_string(&mut self) -> Result<(), ParserError> {
        // now this function can group all string in  the program
        let mut group: String = String::new();
        let mut rlist: Vec<ExprElem> = Vec::new();
        let mut open_status: StringAreaState = StringAreaState::Closed;
        let mut ignore_flag = false;
        let mut string_escape_flag = false;

        for (count, inner) in self.code_list.iter().enumerate() {
            if ignore_flag {
                ignore_flag = false;
                // 二文字の判別
                continue;
            }
            if let ExprElem::UnKnownElem(e) = inner {
                match open_status {
                    StringAreaState::CommentStart => {
                        // //が開いているとき
                        if e.contents == '\n' {
                            rlist.push(ExprElem::CommentElem(CommentBranch {
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
                                if let ExprElem::UnKnownElem(next_e) = &self.code_list[count + 1] {
                                    if Self::COMMENT_CLOSE.ends_with(next_e.contents)
                                    // "/" == e.content
                                    {
                                        rlist.push(ExprElem::CommentElem(CommentBranch {
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
                                    return Err(ParserError::UnexpectedTypeExpr);
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
                                rlist.push(ExprElem::StringElem(StringBranch {
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
                                if let ExprElem::UnKnownElem(next_e) = &self.code_list[count + 1] {
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
            rlist.push(ExprElem::CommentElem(CommentBranch {
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
                        _ => return Err(ParserError::BraceNotOpened),
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
                                return Err(ParserError::UnexpectedTypeExpr);
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

    fn grouping_operator(&mut self) -> Result<(), ParserError> {
        for ope in Self::LENGTH_ORDER_OPE_LIST {
            self.grouping_operator_unit(ope.opestr.to_string())?;
        }
        Ok(())
    }

    fn grouping_operator_unit(&mut self, ope: String) -> Result<(), ParserError> {
        let mut group: String = String::new();
        let mut rlist: Vec<ExprElem> = Vec::new();

        let ope_size: usize = ope.len();
        for inner in &self.code_list {
            if let ExprElem::UnKnownElem(e) = inner {
                // 未解決の場合
                group.push(e.contents);
                match group.len().cmp(&ope_size) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        if group == ope {
                            rlist.push(ExprElem::OpeElem(OperatorBranch {
                                ope: group.clone(),
                                depth: self.depth,
                            }))
                        } else {
                            let group_tmp = Self::code2_vec_pre_proc_func(&group);
                            rlist.extend(group_tmp);
                        }
                        group.clear();
                    }
                    Ordering::Greater => {
                        // ope_size < group.len()
                        // rlist += group
                        let group_tmp = Self::code2_vec_pre_proc_func(&group);
                        rlist.extend(group_tmp);
                        group.clear();
                    }
                }
            } else {
                // 既にtokenが割り当てられているとき
                match group.len().cmp(&ope_size) {
                    Ordering::Less => {
                        let group_tmp = Self::code2_vec_pre_proc_func(&group);
                        rlist.extend(group_tmp);
                    }
                    Ordering::Equal => {
                        if group == ope {
                            rlist.push(ExprElem::OpeElem(OperatorBranch {
                                ope: group.clone(),
                                depth: self.depth,
                            }))
                        } else {
                            // rlist += group
                            let group_tmp = Self::code2_vec_pre_proc_func(&group);
                            rlist.extend(group_tmp);
                        }
                    }
                    Ordering::Greater => {
                        // rlist += group
                        let group_tmp = Self::code2_vec_pre_proc_func(&group);
                        rlist.extend(group_tmp);
                    }
                }
                group.clear();
                rlist.push(inner.clone());
            }
        } //end of "for inner in codelist"
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_syntax(&mut self) -> Result<(), ParserError> {
        let mut name: Option<String> = None;
        let mut expr: Option<ParenBlockBranch> = None;
        let mut rlist: Vec<ExprElem> = Vec::new();

        for inner in &self.code_list {
            if let ExprElem::WordElem(wd) = inner {
                if Self::SYNTAX_WORDS.contains(&wd.contents.as_str()) {
                    name = Some(wd.contents.clone());
                } else {
                    rlist.push(inner.clone());
                }
            } else if let ExprElem::ParenBlockElem(pb) = inner {
                if name.is_some() {
                    expr = Some(pb.clone());
                } else {
                    rlist.push(inner.clone());
                }
            } else if let ExprElem::BlockElem(bl) = inner {
                if let Some(syntax_name) = name {
                    rlist.push(ExprElem::SyntaxElem(SyntaxBranch {
                        name: syntax_name,
                        expr: if let Some(syntax_expr) = expr {
                            syntax_expr.contents
                        } else {
                            Vec::new()
                        },
                        contents: bl.contents.clone(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                } else {
                    // TODO
                    // error とは限らない
                    if let Some(syntax_expr) = expr {
                        rlist.push(ExprElem::ParenBlockElem(syntax_expr));
                    }
                    rlist.push(inner.clone());
                }
                name = None;
                expr = None;
            } else {
                // name expr をError処理
                rlist.push(inner.clone());
            }
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_syntaxbox(&mut self) -> Result<(), ParserError> {
        let mut flag = false;
        let mut name: String = String::new();
        let mut group: Vec<SyntaxBranch> = Vec::new();
        let mut rlist: Vec<ExprElem> = Vec::new();

        for inner in &self.code_list {
            if let ExprElem::SyntaxElem(ref e) = inner {
                if Self::SYNTAX_WORDS_HEADS.contains(&e.name.as_str()) {
                    flag = true;
                    name.clone_from(&e.name);
                    group.push(e.clone());
                } else if e.name == Self::SYNTAX_ELIF {
                    if flag {
                        group.push(e.clone());
                    } else {
                        return Err(ParserError::GroupingSyntaxBoxError);
                    }
                } else if e.name == Self::SYNTAX_ELSE {
                    if flag {
                        group.push(e.clone());
                        rlist.push(ExprElem::SyntaxBoxElem(SyntaxBoxBranch {
                            name: name.clone(),
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                        name = String::from("");
                        flag = false;
                    } else {
                        return Err(ParserError::GroupingSyntaxBoxError);
                    }
                } else {
                    rlist.push(inner.clone());
                }
            } else {
                if flag {
                    if !group.is_empty() {
                        rlist.push(ExprElem::SyntaxBoxElem(SyntaxBoxBranch {
                            name: name.clone(),
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                        name = String::from("");
                    } else {
                        //pass
                    }
                    flag = false;
                } else {
                    //pass
                }
                rlist.push(inner.clone());
            }
        }
        if !group.is_empty() {
            rlist.push(ExprElem::SyntaxBoxElem(SyntaxBoxBranch {
                name: name.clone(),
                contents: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        self.code_list = rlist;
        Ok(())
    }

    //
    // TODO: Word以外について`()`が付与され呼ばれたときに
    // 関数として認識できるようにする必要がある
    // 例えば以下のような場合について
    // ```lichen
    // funcA()() // 関数を返却するような関数
    // a[]()     // 関数を保持しているリスト
    // ```
    fn contain_subscriptable(&self) -> bool {
        let mut name_tmp: Option<&ExprElem> = None;

        for inner in &self.code_list {
            if let ExprElem::WordElem(_)
            | ExprElem::FuncElem(_)
            | ExprElem::ListElem(_)
            | ExprElem::SyntaxBoxElem(_) = inner
            {
                name_tmp = Some(inner);
            } else if let ExprElem::ListBlockElem(_) | ExprElem::ParenBlockElem(_) = inner {
                if let Some(ExprElem::WordElem(v)) = name_tmp {
                    if !Self::KEYWORDS.contains(&v.contents.as_str()) {
                        return true;
                    }
                } else if let Some(
                    ExprElem::FuncElem(_) | ExprElem::ListElem(_) | ExprElem::SyntaxBoxElem(_),
                ) = name_tmp
                {
                    return true;
                } else {
                    name_tmp = None;
                }
            } else if name_tmp.is_some() {
                name_tmp = None;
            }
        }
        false
    }

    fn grouping_subscription(&mut self) -> Result<(), ParserError> {
        let mut name_tmp: Option<ExprElem> = None;
        let mut rlist: Vec<ExprElem> = Vec::new();

        for inner in &self.code_list {
            if let ExprElem::WordElem(_)
            | ExprElem::FuncElem(_)
            | ExprElem::ListElem(_)
            | ExprElem::SyntaxBoxElem(_) = inner
            {
                if let Some(v) = name_tmp {
                    rlist.push(v);
                }
                name_tmp = Some(inner.clone());
            }
            // [] ()
            else if let Some(v) = &name_tmp {
                if let ExprElem::WordElem(ref wd) = v {
                    if !Self::KEYWORDS.contains(&wd.contents.as_str()) {
                        // jump to point01
                    } else {
                        // 1
                        rlist.push(v.clone());
                        rlist.push(inner.clone());
                        name_tmp = None;
                        continue;
                    }
                } else if let ExprElem::FuncElem(_)
                | ExprElem::ListElem(_)
                | ExprElem::SyntaxBoxElem(_) = &v
                {
                    // jump to point01
                } else {
                    rlist.push(v.clone());
                    rlist.push(inner.clone());
                    name_tmp = None;
                    continue;
                }
                // point01
                if let ExprElem::ListBlockElem(_) = inner {
                    rlist.push(ExprElem::ListElem(ListBranch {
                        name: Box::new(v.clone()),
                        contents: vec![inner.clone()],
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                } else if let ExprElem::ParenBlockElem(_) = inner {
                    rlist.push(ExprElem::FuncElem(FuncBranch {
                        name: Box::new(v.clone()),
                        contents: vec![inner.clone()],
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                } else {
                    rlist.push(v.clone());
                    rlist.push(inner.clone());
                }
                name_tmp = None;
            } else {
                rlist.push(inner.clone());
            }
        }
        if let Some(v) = &name_tmp {
            rlist.push(v.clone());
        }
        self.code_list = rlist;
        Ok(())
    }

    fn find_min_priority_index(&self) -> Result<Option<usize>, ParserError> {
        let mut priority_tmp: i32 = i32::MAX;
        let mut index_tmp = None;

        for (index, inner) in self.code_list.iter().enumerate() {
            if let ExprElem::OpeElem(ope) = inner {
                let ope_contents = &ope.ope;
                if let Ok(ope_info) = Self::find_ope_priority(ope_contents) {
                    if index < 1
                    // if index == 0:
                    {
                        index_tmp = Some(index);
                        priority_tmp = 4; // unsafe
                    } else if let ExprElem::OpeElem(_) = &self.code_list[index - 1] {
                        continue;
                    } else if ope_info.priority < priority_tmp {
                        index_tmp = Some(index);
                        priority_tmp = ope_info.priority;
                    } else if ope_info.priority == priority_tmp {
                        match ope_info.priority_direction {
                            Prio::Left => {
                                index_tmp = Some(index);
                                priority_tmp = ope_info.priority;
                            }
                            Prio::Right => {}
                            Prio::Prefix => {}
                        }
                    } // else pass
                } else {
                    // error case
                    return Err(ParserError::OperationError);
                }
            } else {
                continue;
            }
        }
        Ok(index_tmp)
    }

    fn resolve_operation(&mut self) -> Result<(), ParserError> {
        let operation_index = self.find_min_priority_index();
        match operation_index {
            Ok(v) => {
                if let Some(s) = v {
                    let arg1 = ExprElem::ItemElem(ItemBranch {
                        contents: self.code_list[..s].to_vec(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    });
                    let name = &self.code_list[s];
                    let arg2 = ExprElem::ItemElem(ItemBranch {
                        contents: self.code_list[s + 1..].to_vec(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    });
                    self.code_list = vec![ExprElem::FuncElem(FuncBranch {
                        name: Box::new(name.clone()),
                        contents: vec![arg1, arg2],
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    })];
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
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

impl Parser<'_> for ExprParser {
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

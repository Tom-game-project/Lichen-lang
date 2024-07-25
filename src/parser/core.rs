use crate::parser::token::*;

/// # Parser trait
pub trait Parser<'a> {
    const LEFT_PRIORITY_LIST: [(&'a str, isize); 14] = [
        ("||", -3),
        ("&&", -2),
        // PRIORITY 0
        ("==", 0),
        ("!=", 0),
        ("<", 0),
        (">", 0),
        (">=", 0),
        ("<=", 0),
        // PRIORITY 1
        ("+", 1),
        ("-", 1),
        // PRIORITY 2
        ("*", 2),
        ("/", 2),
        ("%", 2),
        ("@", 2),
    ];
    const RIGHT_PRIORITY_LIST: [(&'a str, isize); 7] = [
        // PRIORITY -4
        ("=", -4),
        ("+=", -4),
        ("-=", -4),
        ("*=", -4),
        ("/=", -4),
        ("%=", -4),
        ("**", 3),
    ];
    const PREFIX_PRIORITY_LIST: [(&'a str, isize); 1] = [
        // PRIORITY -1
        ("!", -1),
    ];
    const SPLIT_CHAR: [char; 3] = [' ', '\t', '\n'];
    const EXCLUDE_WORDS: [&'a str; 3] = [";", ":", ","];
    const SYNTAX_WORDS: [&'a str; 7] = ["if", "elif", "else", "loop", "for", "while", "match"];
    const SYNTAX_WORDS_HEADS: [&'a str; 4] = ["if", "loop", "for", "while"];
    const ESCAPECHAR: char = '\\';
    const FUNCTION: &'a str = "fn";
    const SEMICOLON: char = ';';

    const CONTROL_STATEMENT: [&'a str; 4] = ["return", "break", "continue", "assert"];

    fn new(code: String, depth: isize, loopdepth: isize) -> Self;
    fn resolve(&self) -> Result<Vec<BaseElem>, String>;
    fn code2vec(&self, code: &Vec<BaseElem>) -> Result<Vec<BaseElem>, &str>;
    fn get_depth(&self) -> isize;
    fn get_loopdepth(&self) -> isize;

    fn code2_vec_pre_proc_func(&self, code: &String) -> Vec<BaseElem> {
        return code
            .chars()
            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
            .collect();
    }

    // grouoping functions
    fn grouping_quotation(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        let mut open_flag = false;
        let mut escape_flag = false;
        let mut rlist = Vec::new();
        let mut group = String::new();

        for inner in codelist {
            if let BaseElem::UnKnownElem(ref v) = inner {
                if escape_flag {
                    group.push(v.contents);
                    escape_flag = false
                } else {
                    if v.contents == '"'
                    // is quochar
                    {
                        if open_flag {
                            group.push(v.contents);
                            rlist.push(BaseElem::StringElem(StringBranch {
                                contents: group.clone(),
                                depth: self.get_depth(),
                            }));
                            group.clear();
                            open_flag = false;
                        } else {
                            group.push(v.contents);
                            open_flag = true;
                        }
                    } else {
                        if open_flag {
                            if v.contents == '\\' {
                                escape_flag = true;
                            } else {
                                escape_flag = false;
                            }
                            group.push(v.contents);
                        } else {
                            rlist.push(inner);
                        }
                    }
                }
            } else {
                rlist.push(inner);
            }
        }
        if open_flag {
            return Err("[Error: quotation is not closed]");
        }
        return Ok(rlist);
    }

    fn grouping_elements<T>(
        &self,
        codelist: Vec<BaseElem>,
        elemtype: fn(T) -> BaseElem,
        open_char: char,
        close_char: char,
    ) -> Result<Vec<BaseElem>, &str>
    where
        T: ASTAreaBranch,
    {
        let mut rlist: Vec<BaseElem> = Vec::new();
        let mut group: Vec<BaseElem> = Vec::new();
        let mut depth: isize = 0;

        for inner in codelist {
            if let BaseElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    if depth > 0 {
                        group.push(inner);
                    } else if depth == 0 {
                        // pass
                    } else {
                        return Err("[Error:]");
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    if depth > 0 {
                        group.push(inner);
                    } else if depth == 0 {
                        rlist.push(elemtype(ASTAreaBranch::new(
                            Some(group.clone()),
                            self.get_depth(),
                            self.get_loopdepth(),
                        )));
                        group.clear();
                    } else {
                        return Err("[Error:]");
                    }
                } else {
                    if depth > 0 {
                        group.push(inner);
                    } else if depth == 0 {
                        rlist.push(inner);
                    } else {
                        return Err("[Error:]");
                    }
                }
            } else {
                if depth > 0 {
                    group.push(inner);
                } else if depth == 0 {
                    rlist.push(inner);
                } else {
                    return Err("[Error:(user error) block must be closed]");
                }
            }
        }
        if depth != 0 {
            return Err("[Error:(user error) block must be closed]");
        }
        return Ok(rlist);
    }

    /// # grouping_word
    /// スペースなどので区切られた単語をまとめる
    fn grouping_word(
        &self,
        codelist: Vec<BaseElem>,
        split: Vec<char>,
        excludes: Vec<char>,
    ) -> Result<Vec<BaseElem>, &str> {
        let mut rlist: Vec<BaseElem> = Vec::new();
        let mut group: String = String::new();

        for inner in codelist {
            if let BaseElem::UnKnownElem(ref e) = inner {
                if split.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        rlist.push(BaseElem::WordElem(WordBranch {
                            contents: group.clone(),
                        }));
                        group.clear();
                    }
                } else if excludes.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        rlist.push(BaseElem::WordElem(WordBranch {
                            contents: group.clone(),
                        }));
                        group.clear();
                    }
                    rlist.push(inner);
                } else {
                    group.push(e.contents);
                }
            } else {
                if !group.is_empty() {
                    rlist.push(BaseElem::WordElem(WordBranch {
                        contents: group.clone(),
                    }));
                    group.clear();
                }
                rlist.push(inner);
            }
        }
        if !group.is_empty() {
            rlist.push(BaseElem::WordElem(WordBranch {
                contents: group.clone(),
            }));
            group.clear();
        }
        return Ok(rlist);
    }

    fn grouping_syntaxbox(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str>;

    ///
    /// TODO: Word以外について`()`が付与され呼ばれたときに
    /// 関数として認識できるようにする必要がある
    /// 例えば以下のような場合について
    /// ```lichen
    /// funcA()() // 関数を返却するような関数
    /// a[]()     // 関数を保持しているリスト
    /// ```
    fn grouping_functioncall<T>(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        let mut flag: bool = false;
        let mut name_tmp: Option<BaseElem> = None;
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in codelist {
            if let BaseElem::WordElem(ref wb) = inner {
                // Case WordElem
                if flag {
                    if let Some(e) = name_tmp {
                        rlist.push(e);
                    }
                }
                name_tmp = Some(inner);
                flag = true;
            } else if let BaseElem::FuncElem(ref fb) = inner {
                // Case FuncElem
                if flag {
                    if let Some(e) = name_tmp {
                        rlist.push(e);
                    }
                }
                name_tmp = Some(inner);
                flag = true;
            } else if let BaseElem::ParenBlockElem(ref pbb) = inner {
                // Case ParenBlockElem
                if flag {
                    if let Some(ref base_e) = name_tmp {
                        if let BaseElem::WordElem(ref wb) = base_e {
                            if <Self as Parser>::CONTROL_STATEMENT.contains(&(&wb.contents as &str))
                            {
                                rlist.push(BaseElem::FuncElem(FuncBranch {
                                    name: Box::new(base_e.clone()),
                                    contents: pbb.clone(),
                                    depth: self.get_depth(),
                                    loopdepth: self.get_loopdepth(),
                                }));
                                name_tmp = None;
                                flag = false;
                            } else {
                                // name tmp is not none
                                rlist.push(base_e.clone()); // contents of name_tmp -> base_e
                                rlist.push(inner);
                                name_tmp = None;
                            }
                        } else if let BaseElem::FuncElem(_) = base_e {
                            rlist.push(BaseElem::FuncElem(FuncBranch {
                                name: Box::new(base_e.clone()),
                                contents: pbb.clone(),
                                depth: self.get_depth(),
                                loopdepth: self.get_loopdepth(),
                            }));
                            name_tmp = None;
                            flag = false;
                        } else {
                            // name tmp is not none
                            rlist.push(base_e.clone()); // contents of name_tmp -> base_e
                            rlist.push(inner);
                            name_tmp = None;
                        }
                    } else {
                        //name tmp is none
                        rlist.push(inner);
                        flag = false;
                        name_tmp = None;
                    }
                }
            } else {
                // pass
            }
        }
        if flag {
            if let Some(e) = name_tmp {
                rlist.push(e);
            }
        }
        return Ok(rlist);
    }
}

pub struct StateParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub depth: isize,
    pub loopdepth: isize,
}

pub struct ExprParser {
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

impl Parser<'_> for ExprParser {
    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: code,
            depth: depth,
            loopdepth: loopdepth,
        }
    }

    fn resolve(&self) -> Result<Vec<BaseElem>, String> {
        // let codelist = self.code2vec(&self.code2_vec_pre_proc_func(code));
        // for i in codelist{

        // }
        // return codelist;
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

    fn grouping_syntaxbox(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        let mut flag = false;
        let mut name: String = String::new();
        let mut group: Vec<SyntaxBranch> = Vec::new();
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in codelist {
            if let BaseElem::SyntaxElem(ref e) = inner {
                if Self::SYNTAX_WORDS_HEADS.contains(&e.name.as_str()) {
                    flag = true;
                    name = e.name.clone();
                    group.push(e.clone());
                } else if e.name == "elif" {
                    if flag {
                        group.push(e.clone());
                    } else {
                        return Err("please write \"if\",\"while\" or \"for\" statement head");
                        // TODO:
                    }
                } else if e.name == "else" {
                    if flag {
                        group.push(e.clone());
                        rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
                            name: name.clone(),
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                        name = String::from("");
                        flag = false;
                    } else {
                        return Err("please write \"if\",\"while\" or \"for\" statement head");
                        // TODO:
                    }
                } else {
                    rlist.push(inner);
                }
            } else {
                if flag {
                    if !group.is_empty() {
                        rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
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
                rlist.push(inner);
            }
        }
        if !group.is_empty() {
            rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
                name: name.clone(),
                contents: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        return Ok(rlist);
    }
}

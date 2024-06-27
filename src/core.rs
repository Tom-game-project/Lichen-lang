#[derive(Clone)]
pub enum BaseElem {
    BlockElem(BlockBranch),
    ListBlockElem(ListBlockBranch),
    ParenBlockElem(ParenBlockBranch),
    StringElem(StringBranch),
    SyntaxElem(SyntaxBranch),
    WordElem(WordBranch),
    UnKnownElem(UnKnownBranch),
}

impl BaseElem {
    pub fn show(&self) {
        match self {
            BaseElem::BlockElem(e) => e.show(),
            BaseElem::UnKnownElem(e) => e.show(),
            BaseElem::StringElem(e) => e.show(),
            BaseElem::ListBlockElem(e) => e.show(),
            BaseElem::ParenBlockElem(e) => e.show(),
            BaseElem::WordElem(e) => e.show(),
            BaseElem::SyntaxElem(e) => e.show(),
        }
    }

    pub fn resolve_self(&mut self) -> Result<&str, String> {
        match self {
            // recursive analysis elements
            BaseElem::BlockElem(e) => return e.resolve_self(),
            BaseElem::ListBlockElem(e) => return e.resolve_self(),
            BaseElem::ParenBlockElem(e) => return e.resolve_self(),
            BaseElem::SyntaxElem(e) => return e.resolve_self(),

            // unrecursive analysis elements
            BaseElem::StringElem(_) => return Ok("Ok"),
            BaseElem::WordElem(_) => return Ok("Ok"),
            BaseElem::UnKnownElem(_) => return Ok("Ok"),
        }
    }
}

pub trait ASTBranch {
    fn show(&self);
}

pub trait ASTAreaBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize) -> Self;
    fn resolve_self(&mut self) -> Result<&str, String>;
}

#[derive(Clone)]
pub struct BlockBranch {
    contents: Option<Vec<BaseElem>>,
    depth: isize,
}

impl ASTAreaBranch for BlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize) -> Self {
        Self {
            contents: contents,
            depth: depth,
        }
    }

    fn resolve_self(&mut self) -> Result<&str, String> {
        if let Some(a) = &self.contents {
            let parser = StateParser::new(String::from(""), self.depth + 1);
            match parser.code2vec(&a) {
                Ok(v) => {
                    let mut rlist = v.to_vec();
                    for i in &mut rlist {
                        match i.resolve_self() {
                            Ok(_) => { /* pass */ }
                            Err(_) => { /* pass */ }
                        };
                    }
                    self.contents = Some(rlist);
                    return Ok("OK!");
                }
                Err(e) => {
                    // println!("{}",e);
                    return Err(String::from(e));
                }
            }
        } else {
            return Ok("Empty");
        }
    }
}

impl ASTBranch for BlockBranch {
    fn show(&self) {
        println!("BlockBranch depth{} (", self.depth);
        if let Some(e) = &self.contents {
            for i in e {
                i.show();
            }
        }
        println!(")");
    }
}

#[derive(Clone)]
pub struct ListBlockBranch {
    contents: Option<Vec<BaseElem>>,
    depth: isize,
}

impl ASTBranch for ListBlockBranch {
    fn show(&self) {
        println!("List depth{} (", self.depth);
        if let Some(e) = &self.contents {
            for i in e {
                i.show();
            }
        }
        println!(")");
    }
}

impl ASTAreaBranch for ListBlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize) -> Self {
        Self {
            contents: contents,
            depth: depth,
        }
    }

    fn resolve_self(&mut self) -> Result<&str, String> {
        //todo!();
        // TODO:impl list parser
        // TODO:impl slice parser
        return Ok("Ok!");
    }
}

#[derive(Clone)]
pub struct ParenBlockBranch {
    contents: Option<Vec<BaseElem>>,
    depth: isize,
}

impl ASTBranch for ParenBlockBranch {
    fn show(&self) {
        println!("Paren depth{} (", self.depth);
        if let Some(e) = &self.contents {
            for i in e {
                i.show();
            }
        }
        println!(")");
    }
}

impl ASTAreaBranch for ParenBlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize) -> Self {
        Self {
            contents: contents,
            depth: depth,
        }
    }
    fn resolve_self(&mut self) -> Result<&str, String> {
        // TODO: impl expr parser
        // TODO: impl args parser
        // TODO: impl tuple parser
        return Ok("Ok!");
    }
}

#[derive(Clone)]
pub struct SyntaxBranch {
    name: String,
    expr: Option<Box<BaseElem>>,
    contents: Option<Vec<BaseElem>>,
    depth: isize,
    loopdepth: isize,
}

impl ASTBranch for SyntaxBranch {
    fn show(&self) {
        todo!()
    }
}

impl ASTAreaBranch for SyntaxBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize) -> Self {
        todo!()
    }

    fn resolve_self(&mut self) -> Result<&str, String> {
        todo!()
    }
}

#[derive(Clone)]
pub struct StringBranch {
    contents: String,
}

impl ASTBranch for StringBranch {
    fn show(&self) {
        println!("String ({})", self.contents);
    }
}

#[derive(Clone)]
pub struct WordBranch {
    contents: String,
}

impl ASTBranch for WordBranch {
    fn show(&self) {
        println!("Word {}", self.contents)
    }
}

#[derive(Clone)]
pub struct UnKnownBranch {
    contents: char,
}

impl ASTBranch for UnKnownBranch {
    fn show(&self) {
        println!("UnKnownBranch :\"{}\"", self.contents);
    }
}

/// # Parser trait
pub trait Parser {
    // const NUM:i32 = 1;
    fn new(code: String, depth: isize) -> Self;

    fn resolve(&self) -> Result<Vec<BaseElem>, String>;
    fn code2vec(&self, code: &Vec<BaseElem>) -> Result<Vec<BaseElem>, &str>;
    fn get_depth(&self) -> isize;

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
}

pub struct StateParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub depth: isize,
}

pub struct ExprParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub depth: isize,
}

impl Parser for StateParser {
    fn new(code: String, depth: isize) -> Self {
        Self {
            code: code,
            depth: depth,
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

    // grouping functions
    fn grouping_syntaxbox(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        todo!()
    }
}

// impl StateParser {
//     fn code2_vec_pre_proc_func(&self, code: &String) -> Vec<BaseElem> {
//         return code
//             .chars()
//             .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
//             .collect();
//     }
// }

impl Parser for ExprParser {
    fn new(code: String, depth: isize) -> Self {
        Self {
            code: code,
            depth: depth,
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

    fn grouping_syntaxbox(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>, &str> {
        let mut flag = false;
        let mut name: String = String::new();
        let mut group = Vec::new();
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in codelist {
            if let BaseElem::SyntaxElem(e) = inner {
                //
            } else {
            }
        }
        if !group.is_empty() {
            //rlist.push(value)
        }
        return Ok(rlist);
    }
}

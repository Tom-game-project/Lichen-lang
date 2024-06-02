use std::char::EscapeUnicode;


struct OpePair(&str, i32);

struct OpePairList{
    data : Vec<OpePair>
}

impl  OpePairList{

    fn new(data:Vec<OpePair>){
        Self {
            data:data
        }
    }

    fn find_priority(&self,key:&str) -> Option<i32>{
        for i in self.data{
            if i.0 == key{
                return Some(i.1); 
            }
        }
        return None;
    }

    fn sort_by_string_length(&mut self){
        self.data.sort_by(|a,b| a.0.len() < b.0.len());
    }

    // debug tools
    fn show(&self){
        for (i,j) in self.data.iter().enumerate(){
            println!("index:{}, ope:{}, priority{}",i, j.0, j.1);
        }
    }

    fn keys(&self) -> Vec<String>{
        let mut rlist = Vec::new();
        for i in self.data{
            rlist.push(i.0.to_string());
        }
        return (rlist);
    }
}

struct Parser {
    // # input
    code:String,
    depth:usize,
    loopdepth:usize,
    // # default settings
    // ## priority
    left_priority_list:OpePairList,
    right_priority_list:OpePairList,
    prefix_priority_list:OpePairList,

    // # proc setting
    split_char:      Vec<char>,
    char_exclude:    Vec<char>,

    // syntax
    syntax_words:          Vec<&str>,
    syntax_word_heads:     Vec<&str>,
    control_statement:     Vec<&str>,
    primitive_object_type: Vec<&str>,

    
    escape_char:char,
    semicolon:char,
    function_string:String,
}

impl Parser{

    fn new(code:String, depth:usize, loopdepth:i32) -> Self{
        // # left_priority_list initialize
        let left_priority_list:OpePairList = OpePairList::new(vec![
            ("||", -3),
            ("&&", -2),
            ("==", 0) ,
            ("!=", 0) ,
            ("<", 0)  ,
            (">", 0)  ,
            ("<=", 0) ,
            (">=", 0) ,
            ("+", 1)  ,
            ("-", 1)  ,
            ("*", 2)  ,
            ("/", 2)  ,
            ("%", 2)  ,
            ("@", 2)  ,
        ]);

        // # left_priority_list initialize
        let right_priority_list:OpePairList = OpePairList::new(vec![
            ("=", -4) ,
            ("+=", -4),
            ("-=", -4),
            ("*=", -4),
            ("/=", -4),
            // ## 二乗
            ("**", 3) ,
        ]);

        // # prefix_priority_list
        let prefix_priority_list:OpePairList = OpePairList::new(vec![
            ("!", -1),
        ]);

        let split_char = vec![' ', '\t', '\n'];
        let char_exclude = vec![';',':',','];

        let syntax_words = vec![
            "if",
            "elif",
            "else",
            "loop",
            "for",
            "while",
        ];
        let syntax_word_heads = vec![
            "if",
            "loop",
            "for",
            "while",
        ];
        let control_statement = vec![
            "return",
            "break",
            "continue",
            "assert",
        ];
        let primitive_object_type = vec![
            "i32",
            "i64",
            "f32",
            "f64",
        ];

        return Self {
            code : code.clone(),
            depth: depth,
            loopdepth: loopdepth,
            //# default settings
            // ## priority 
            left_priority_list: left_priority_list,
            right_priority_list: right_priority_list,
            prefix_priority_list:prefix_priority_list,
            // syntax
            syntax_words:syntax_words,
            syntax_word_heads:syntax_word_heads,
            control_statement:control_statement,
            primitive_object_type:primitive_object_type,
            // proc
            split_char : split_char,
            char_exclude : char_exclude,

            escape_char   : '\\',
            function_string : String::from("fn"),
            semicolon : ';',
        };

    }

    fn grouping_quotation(&self, code:Vec<Elem>, quo_char:char) -> Result<Vec<Elem>,&str>{
        let mut open_flag = false;
        let mut escape_flag = false;
        let mut rlist:Vec<Elem> = Vec::new();
        let mut group:String = Vec::new();

        for inner in code{
            if escape_flag{
                group.push(inner);
                escape_flag = false
            }else{
                match inner{
                    Elem::UNKNOWN(c) => {
                        if c == quo_char
                        {
                            if open_flag
                            {
                                group.push(c);
                                rlist.push(
                                    Elem::ElemString(
                                        StructString{
                                            contents:group
                                        }
                                    )
                                );
                                group.clear();
                                open_flag = false;
                            }
                            else
                            {
                                group.append(c);
                                open_flag = true;
                            }
                        }
                        else
                        {
                            if open_flag
                            {
                                if c == self.escape_char
                                {
                                    escape_flag = true;
                                }
                                else
                                {
                                    escape_flag = false;
                                }
                                group.push(c);
                            }
                            else
                            {
                                rlist.push(inner);
                            }
                        }
                    }
                    _ =>{
                        rlist.push(inner); 
                    }
                }
            }
        }
        //error check proc
        if open_flag
        {
            return Err("you must close quotation");
        }
        return Ok(rlist);
    }

	fn grouping_elements(&self, code:Vec<Elem>, open_char:char, close_char:char,object_instance:Elem) -> Result<Vec<Elem>,&str>{
        let mut rlist:Vec<Elem> = Vec::new();
        let mut group:Vec<Elem> = Vec::new();
        let mut depth:i32 = 0;

        for inner in code{
            match inner
            {
                Elem::UNKNOWN(c ) => {
                    if c == open_char
                    {
                        if 0 < depth
                        {
                            group.push(Elem::UNKNOWN(c));
                        }
                        else if depth == 0
                        {
                            //pass
                        }
                        else
                        {
                            return Err("User Error:invalid syntax Error : depth");
                        }
                        depth += 1;
                    }
                    else if c == close_char
                    {
                        depth -= 1;
                        if 0 < depth
                        {
                            group.push(inner);
                        }
                        else if depth == 0
                        {
                            match object_instance
                            {
                                Elem::ElemBlock =>
                                {
                                    rlist.push(Elem::ElemBlock(StructBlock{
                                        contents:group,
                                        depth:self.depth,
                                        loopdepth:self.loopdepth
                                    }));
                                }
                                Elem::ElemParenBlock =>
                                {
                                    rlist.push(Elem::ElemParenBlock(StructParenBlock{
                                        contents:group,
                                        depth:self.depth,
                                        loopdepth:self.loopdepth
                                    }));
                                }
                                Elem::ElemListBlock =>
                                {
                                    rlist.push(Elem::ElemListBlock(StructListBlock {
                                        contents: group
                                    }));
                                }
                                _ => 
                                {
                                    return Err("invalid object instance");
                                }
                            }
                            group.clear();
                        }
                        else
                        {
                            return Err("User Error:invalid syntax Error : depth");
                        }
                    }
                    else
                    {
                        if depth > 0
                        {
                            group.push(inner);
                        }
                        else if  depth == 0
                        {
                            rlist.push(inner);
                        }
                        else
                        {
                            return Err("");
                        }
                    }
                }
                _=>{
                    rlist.push(inner); 
                }
            }
        }
        return Ok(rlist);
    }

    fn grouping_words(&self, code:Vec<Elem>, split:Vec<char>, excludes:Vec<String>) -> Result<Vec<Elem>, &str>{
        let mut rlist :Vec<Elem>= Vec::new();
        let mut group :String = String::new();
        let mut ope_chars:String =  self.left_priority_list.keys() + self.right_priority_list.keys() + excludes;

        for inner in code{
            match inner{
                Elem::UNKNOWN(c) => {
                    if split.contains(&c)
                    {
                        if !group.is_empty()
                        {
                            rlist.push(
                                Elem::ElemWord(
                                    StructWord {
                                        contents: group.join(""),
                                        depth:self.depth
                                    }
                                )
                            );
                            group.clear();
                        }
                    }
                    else if ope_chars.contains(&c)
                    {
                        if !group.is_empty()
                        {
                            rlist.push(
                                Elem::ElemWord(
                                    StructWord {
                                        contents: group.join(""),
                                        depth:self.depth
                                    }
                                )
                            );
                            group.clear();
                        }
                        rlist.push(inner);
                    }
                    else
                    {
                        group.push(c);
                    }
                }
                _ => {
                    // 既に role 決定済み
                    if !group.is_empty()
                    {
                        rlist.push(
                            Elem::ElemWord(
                                StructWord {
                                    contents: group.join(""),
                                    depth:self.depth
                                }
                            )
                        );
                        group.clear();
                    }
                    rlist.push(inner);
                }
            }
        }
        if !group.is_empty()
        {
            rlist.push(
                Elem::ElemWord(
                    StructWord {
                        contents: group.join(""),
                        depth:self.depth
                    }
                ));
            group.clear();
        }
        return Ok(rlist);
    }

    fn grouping_syntax(&self, code:Vec<Elem>, syntax_words:Vec<String>) -> Result<Vec<Elem>,&str>{
        let mut flag = false;
        let mut rlist :Vec<Elem> = Vec::new();
        let mut group :Vec<Elem> = Vec::new();

        for inner in code{
            match inner {
                Elem::ElemWord(w) => {
                    if syntax_words.contains(&w.contents) 
                    {
                        group.push(inner);
                        flag = true;
                    }
                    else
                    {
                        rlist.push(inner);
                    }
                }
                Elem::ElemParenBlock(_) => {
                    if flag
                    {
                        group.push(inner);
                    }
                    else {
                        rlist.push(inner);
                    }
                }
                Elem::ElemBlock(b) => {
                    if flag
                    {
                        group.push(inner);
                        if group.len() == 2
                        {
                            let name:String;
                            let block:Vec<Elem>;
                            match group[0]{
                                Elem::ElemWord(w) => {
                                    name = w.contents;
                                }
                                _=>{
                                    return Err("Dev Error: grouping_syntax");
                                }
                            }
                            match group[1]{
                                Elem::ElemBlock(b)=>{
                                    block = b.contents;
                                }
                                _ => {
                                    return Err("Dev Error : grouping_syntax");
                                }
                            }
                            let block:Elem::ElemBlock = group[1];
                            rlist.push(Elem::ElemSyntax(
                                StructSyntax{
                                    name:name,
                                    expr:None,
                                    contents:block,
                                    depth:self.depth,
                                    loopdepth:self.loopdepth
                                }
                            ));
                        }
                        else if group.len() == 3
                        {
                            let syntax_name: String;
                            let paren:Vec<Elem>;
                            let block:Vec<Elem>;
                            match group[0]{
                                Elem::ElemWord(w) => {
                                    syntax_name = w.contents;
                                }
                                _ => {
                                    return Err("Dev Error : grouping_syntax");
                                }
                            }
                            match group[2]{
                                Elem::ElemParenBlock(p) => {
                                    paren = p.contents;
                                }
                                _ => {
                                    return Err("Dev Error : grouping_syntax");
                                }
                            }
                            match group[2]{
                                Elem::ElemBlock(b) => {
                                    block = b.contents;
                                }
                                _ => {
                                    return Err("Dev Error : grouping_syntax");
                                }
                            }
                            rlist.push( 
                                Elem::ElemSyntax(
                                    StructSyntax {
                                        name: syntax_name,
                                        expr: Some(paren),
                                        contents: block,
                                        depth: self.depth,
                                        loopdepth: self.loopdepth,
                                    }
                                )
                            );
                        }
                        else
                        {
                            return Err("Dev error : grouping_syntax");
                        }
                        group.clear();
                        flag = false;
                    }
                    else
                    {
                        rlist.push(inner);
                    }
                }
                _ => {
                    rlist.push(inner);
                }
            }
        }
        return Ok(rlist);
    }

    fn grouping_functioncall(&self, code:Vec<Elem>) -> Result<Vec<Elem>,&str>{
        let mut flag = false;
        let mut name_tmp:Option<StructWord> = None;
        let mut rlist:Vec<Elem> = Vec::new();

        for inner in code{
            match inner{
                Elem::ElemWord(w) => {
                    if flag
                    {
                        match name_tmp{
                            Some(e)=>{
                                rlist.push(name_tmp);
                            }
                            None=>{
                                return Err("Dev Error : grouping_functioncall");
                            }
                        }
                    }
                    name_tmp = Some(inner);
                    flag = true;
                }
                Elem::ElemBlock(b) => {
                    match name_tmp {
                        Some(e) => {
                            if flag && self.control_statement.contains(e.contents)
                            {
                                rlist.push(
                                    Elem::ElemFunc(
                                        StructFunc {
                                            name: e, 
                                            contents: b.contents, 
                                            depth: self.depth, 
                                            loopdepth: self.loopdepth
                                        }
                                    )
                                );
                                name_tmp = None;
                                flag = false;
                            }
                            else
                            {
                                rlist.push(e);
                                rlist.push(inner);
                                name_tmp = None;
                            }
                        }
                        None => {
                            // ここの処理は後で考える
                            todo!()
                        }
                    }
                }
                _ => {
                    if flag
                    {
                        match name_tmp{
                            Some(e) => {
                                rlist.push(e);
                            }
                            None => {
                                // pass
                            }
                        }
                        rlist.push(inner);
                        flag = false;
                        name_tmp = None;
                    }
                    else
                    {
                        rlist.push(inner);
                    }
                }
            }
        }
        if flag
        {
            match name_tmp{
                Some(e) =>{
                    rlist.push(e);
                }
                None => {
                    // pass
                }
            }
        }
        return Ok(rlist);
    }

    fn code2vec(&self,code:Vec<Elem>) -> Vec<Elem>
    {
        todo!();   
    }
}

enum Elem{
    // unknown
    UNKNOWN(char),
    // 決定
    ElemBlock(StructBlock),
    ElemString(StructString),
    ElemListBlock(StructListBlock),
    ElemParenBlock(StructParenBlock),
    ElemWord(StructWord),
    ElemSyntax(StructSyntax),
    ElemFunc(StructFunc),
}

struct StructBlock{
    contents:Vec<Elem>,
    depth: i32,
    loopdepth:i32
}

struct StructParenBlock{
    contents:Vec<Elem>,
    depth: i32,
    loopdepth: i32,
}

struct StructListBlock{
    contents:Vec<Elem>
}

struct StructString{
    contents:String
}

struct StructWord{
    contents:String,
    depth:i32
}

struct StructSyntax{
    name:String,
    contents:Vec<Elem>,
    expr:Option<Vec<Elem>>,
    depth: i32,
    loopdepth:i32,
}

struct StructFunc {
    name    :String,
    contents:Vec<Elem>,
    depth:i32,
    loopdepth:i32,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

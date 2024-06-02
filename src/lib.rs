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
                            return Err("invalid syntax Error");
                        }
                        depth += 1;
                    }
                    else if c == close_char
                    {
                        depth -= 1;
                        if 0 < depth
                        {
                            group.push(c);
                        }
                        else if depth == 0
                        {
                            match object_instance
                            {
                                Elem::ElemBlock =>
                                {
                                    rlist.push(Elem::ElemBlock(StructBlock{
                                        contents:group
                                    }));
                                }
                                Elem::ElemParenBlock =>
                                {
                                    rlist.push(Elem::ElemParenBlock(StructParenBlock{
                                        contents:group
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
}

enum Elem{
    // unknown
    UNKNOWN(char),
    // 決定
    ElemBlock(StructBlock),
    ElemString(StructString),
    ElemListBlock(StructListBlock),
    ElemParenBlock(StructParenBlock),
}

struct StructBlock{
    contents:Vec<Elem>
}

struct StructParenBlock{
    contents:Vec<Elem>
}

struct StructListBlock{
    contents:Vec<Elem>
}

struct StructString{
    contents:String
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

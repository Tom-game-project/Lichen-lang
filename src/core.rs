
#[derive(Clone)]
pub enum BaseElem
{
    BlockElem(BlockBranch),
    StringElem(StringBranch),
    UnKnownElem(UnKnownBranch),
}

impl BaseElem
{
    pub fn show(&self) 
    {
        match self
        {
            BaseElem::BlockElem(e) =>
            {
                e.show();
            }
            BaseElem::UnKnownElem(e) =>
            {
                e.show();
            }
            BaseElem::StringElem(e) => 
            {
                e.show();
            }
        }
    }

    pub fn resolve_self(&mut self) -> Result<&str,String>
    {
        match self
        {
            BaseElem::BlockElem(e) => 
            {
                match e.resolve_self()
                {
                    Ok(_) =>
                    {
                        return Ok("Ok")
                    }
                    Err(e) =>
                    {
                        return Err(e);
                    }
                }
            }

            // not recursive elements 
            BaseElem::StringElem(_) => 
            {
                return Ok("Ok");
            }
            BaseElem::UnKnownElem(_) =>
            {
                // pass
                // or return _.resolve_self()
                return  Ok("Ok");
            }
        }
    }
}

pub trait ASTBranch
{
    fn show(&self);
}

pub trait ASTAreaBranch
{
    fn new(contents:Option<Vec<BaseElem>>, depth:isize) -> Self;
    fn resolve_self(&mut self) -> Result<&str,String>;
}

#[derive(Clone)]
pub struct BlockBranch
{
    contents: Option<Vec<BaseElem>>,
    depth:isize
}

impl ASTAreaBranch for BlockBranch
{
    fn new(contents:Option<Vec<BaseElem>>, depth:isize) -> Self {
        Self {
            contents: contents,
            depth: depth
        }
    }
    fn resolve_self(&mut self) -> Result<&str,String>{
        match &self.contents {
            Some(a) => {
                let parser = Parser::new(
                    String::from(""),
                    self.depth + 1
                );
                match parser.code2vec(&a) {
                    Ok(v) => {
                        let mut rlist = v.to_vec();
                        for i in &mut rlist{
                            match i.resolve_self()
                            {
                                Ok(_) => {/* pass */},
                                Err(_) => {/* pass */}
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
            } 
            None => {
                return Ok("Empty");
            }
        }
    }
}

impl ASTBranch for BlockBranch
{
    fn show(&self)
    {
        println!("BlockBranch depth{} (", self.depth);
        match &self.contents
        {
            Some(e) => 
            {
                for i in e
                {
                    i.show();
                }
            }
            None => {/* pass */}
        }
        println!(")");
    }

}


#[derive(Clone)]
pub struct StringBranch
{
    contents: String
}

impl ASTBranch for StringBranch
{
    fn show(&self) {
        println!("String ({})",self.contents);
    }
}

#[derive(Clone)]
pub struct UnKnownBranch
{
    contents: char
}

impl ASTBranch for UnKnownBranch
{
    fn show(&self)
    {
        println!("UnKnownBranch :\"{}\"", self.contents);
    }
}


pub struct Parser
{
    // TODO: 一時的にpublicにしているだけ
    pub code:String,
    pub depth:isize
}

/// # Parser
impl Parser
{
    pub fn new(code:String,depth:isize) -> Self
    {
        Self
        {
            code: code,
            depth:depth
        }
    }

    pub fn resolve(&self) -> Result<Vec<BaseElem>,String>
    {
        let code_list = self.code2_vec_pre_proc_func(&self.code);
        let code_list = self.code2vec(&code_list);
        match code_list{
            Ok(mut v) => 
            {
                for i in &mut v
                {
                    match i.resolve_self()
                    {
                        Ok(_) => {/* pass */}
                        //Err(e) => return Err(e)
                        Err(_) => {/* pass */}
                    }
                    
                }
                return Ok(v);
            }
            Err(e) => 
            {
                return Err(String::from(e));
            }
        }
    }

    fn code2vec(&self,code: &Vec<BaseElem>) -> Result<Vec<BaseElem>,&str>
    {
        let mut code_list;
        //code_list = self.code2_vec_pre_proc_func(&code);
        match self.grouping_quotation(code.to_vec())
        {
            Ok(r) => code_list = r,
            Err(e) => return Err(e)
        }
        match self.grouping_elements(
            code_list,
            BaseElem::BlockElem,
            '{',
            '}'
        )
        {
            Ok(r) => code_list = r,
            Err(e) => return Err(e)
        }
        return Ok(code_list);
    }


    fn code2_vec_pre_proc_func(&self, code:&String) -> Vec<BaseElem>
    {
        return code
                    .chars()
                    .map(|c|BaseElem::UnKnownElem(UnKnownBranch{contents: c}))
                    .collect();
    }


    fn grouping_quotation(&self, codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>,&str>
    {
        let mut open_flag = false;
        let mut escape_flag = false;
        let mut rlist = Vec::new();
        let mut group = String::new();

        for inner in codelist
        {
            match inner
            {
                BaseElem::UnKnownElem(ref v)=>
                {
                    if escape_flag
                    {
                        group.push(v.contents);
                        escape_flag = false
                    }
                    else
                    {
                        if v.contents == '"' // is quochar 
                        {
                            if open_flag
                            {
                                group.push(v.contents);
                                rlist.push(
                                    BaseElem::StringElem(
                                        StringBranch
                                        {
                                            contents: group.clone()
                                        }
                                    )
                                );
                                group.clear();
                                open_flag = false;
                            }
                            else
                            {
                                group.push(v.contents);
                                open_flag = true;
                            }
                        }
                        else
                        {
                            if open_flag
                            {
                                if v.contents == '\\'
                                {
                                    escape_flag = true;
                                }
                                else
                                {
                                    escape_flag = false;
                                }
                                group.push(v.contents);
                            }
                            else
                            {
                                rlist.push(inner);    
                            }
                        }
                    }
                }
                BaseElem::StringElem(_) => 
                {
                    // error
                    //return Err("[Error:]Unreacable");
                    rlist.push(inner);
                }
                BaseElem::BlockElem(_) =>
                {
                    //return Err("[Error:]Unreacable");
                    rlist.push(inner);
                }
            }
        }
        if open_flag
        {
            return Err("[Error: quotation is not closed]");
        }
        return Ok(rlist);
    }

    fn grouping_elements<T>(&self,codelist: Vec<BaseElem>, elemtype:fn(T) -> BaseElem, open_char:char, close_char:char) -> Result<Vec<BaseElem>,&str>
    where T:ASTAreaBranch 
    {
        let mut rlist:Vec<BaseElem> = Vec::new();
        let mut group:Vec<BaseElem> = Vec::new();
        let mut depth:isize = 0;
        for inner in codelist
        {
            match inner
            {
                BaseElem::UnKnownElem(ref b) => {
                    if b.contents == open_char
                    {
                        if depth > 0
                        {
                            group.push(inner);
                        }
                        else if depth == 0
                        {
                            // pass 
                        }
                        else
                        {
                            return Err("[Error:]");
                        }
                        depth += 1;
                    }
                    else if b.contents == close_char
                    {
                        depth -= 1;
                        if depth > 0
                        {
                            group.push(inner);
                        }
                        else if depth == 0
                        {
                            rlist.push(
                                /*elemtype
                                (
                                    
                                    BlockBranch
                                    {
                                        //undec_contents: None,
                                        contents: Some(group.clone()),
                                        depth: self.depth
                                    }
                                )*/
                                elemtype (
                                    ASTAreaBranch::new(
                                        Some(group.clone()),
                                        self.depth
                                    )
                                )
                            );
                            group.clear();
                        }
                        else
                        {
                            return Err("[Error:]");
                        }
                    }
                    else
                    {
                        if depth > 0
                        {
                            group.push(inner);
                        }
                        else if depth == 0
                        {
                            rlist.push(inner);
                        }
                        else
                        {
                            return Err("[Error:]");
                        }
                    }
                }
                BaseElem::StringElem(_)=> {
                    if depth > 0
                    {
                        group.push(inner);
                    }
                    else if depth == 0
                    {
                        rlist.push(inner);
                    }
                    else
                    {
                        return Err("[Error:]");
                    }
                }
                BaseElem::BlockElem(_) => {
                    // pass
                    return Err("[Error:]");
                }
            }
        }
        return Ok(rlist);
    }

}

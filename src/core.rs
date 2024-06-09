

pub enum BaseElem
{
    BlockElem(BlockBranch),
    UnKnownElem(UnKnownBranch)
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
        }
    }
}

pub trait ASTBranch
{
    fn show(&self);
}


pub struct BlockBranch
{
    undec_contents: Option<String>,
    contents: Option<Box<BaseElem>>
}

impl ASTBranch for BlockBranch
{
    fn show(&self)
    {
        match &self.undec_contents
        {
            Some(e) => 
            {
                println!("undec_contents :{}", e);
            }
            None => {/* pass */}
        }
    }
}

pub struct UnKnownBranch
{
    contents: char
}

impl ASTBranch for UnKnownBranch
{
    fn show(&self)
    {
        println!("{}", self.contents);
    }
}
pub struct Parser
{
    // TODO: 一時的にpublicにしているだけ
    pub code:String
}

impl Parser
{
    pub fn new(code:String) -> Self
    {
        Self
        {
            code: code
        }
    }

    pub fn code2vec(&self,code: &str) -> Vec<BaseElem>
    {
        todo!();
    }


    pub fn code2_vec_pre_proc_func(&self, code:&String) -> Vec<BaseElem>
    {
        let mut rlist :Vec<BaseElem>= Vec::new();
        return code
                    .chars()
                    .map(|c|BaseElem::UnKnownElem(UnKnownBranch{contents: c}))
                    .collect();
    }

    pub fn grouping_block(&self,codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>,&str>{
        let mut rlist:Vec<BaseElem> = Vec::new();
        let mut group:String = String::new();
        let mut depth:isize = 0;

        for inner in codelist
        {
            match inner
            {
                BaseElem::UnKnownElem(b) => {
                    if b.contents == '{'
                    {
                        if depth > 0
                        {
                            group.push(b.contents);
                        }
                        else if depth == 0
                        {
                            // pass 
                        }
                        else
                        {
                            return Err("Error!");
                        }
                        depth += 1;
                    }
                    else if b.contents == '}'
                    {
                        depth -= 1;
                        if depth > 0
                        {
                            group.push(b.contents);
                        }
                        else if depth == 0
                        {
                            rlist.push(
                                BaseElem::BlockElem
                                (
                                    BlockBranch
                                    {
                                        undec_contents: Some(group.clone()),
                                        contents:None
                                    }
                                )
                            );
                            group.clear();
                        }
                        else
                        {
                            return Err("Error!");
                        }
                    }
                    else
                    {
                        if depth > 0
                        {
                            group.push(b.contents);
                        }
                        else if depth == 0
                        {
                            rlist.push(BaseElem::UnKnownElem(b));
                        }
                        else
                        {
                            return Err("Error!");
                        }
                    }
                }
                BaseElem::BlockElem(b) => {
                    // pass
                }
            }
        }
        return Ok(rlist);
    }
}

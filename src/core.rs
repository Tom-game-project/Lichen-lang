
#[derive(Clone)]
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

#[derive(Clone)]
pub struct BlockBranch
{
    contents: Option<Vec<BaseElem>>
}

impl ASTBranch for BlockBranch
{
    fn show(&self)
    {
        println!("BlockBranch (");
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

    pub fn resolve(&mut self)
    {
        todo!()
    }

    pub fn code2vec(&self,code: String) -> Result<Vec<BaseElem>,&str>
    {
        let mut code_list;
        code_list = self.code2_vec_pre_proc_func(&code);
        match self.grouping_block(code_list){
            Ok(r) => code_list = r,
            Err(e) => return Err(e)
        }
        return Ok(code_list);
    }


    pub fn code2_vec_pre_proc_func(&self, code:&String) -> Vec<BaseElem>
    {
        return code
                    .chars()
                    .map(|c|BaseElem::UnKnownElem(UnKnownBranch{contents: c}))
                    .collect();
    }

    pub fn grouping_block(&self,codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>,&str>{
        let mut rlist:Vec<BaseElem> = Vec::new();
        let mut group:Vec<BaseElem> = Vec::new();
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
                            group.push(BaseElem::UnKnownElem(b));
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
                            group.push(BaseElem::UnKnownElem(b));
                        }
                        else if depth == 0
                        {
                            rlist.push(
                                BaseElem::BlockElem
                                (
                                    BlockBranch
                                    {
                                        //undec_contents: None,
                                        contents:Some(group.clone())
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
                            group.push(BaseElem::UnKnownElem(b));
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

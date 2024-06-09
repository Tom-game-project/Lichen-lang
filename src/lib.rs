
enum BaseElem{
    BlockElem(BlockBranch),
    UnKnownElem(UnKnownBranch)
}

impl BaseElem{
    fn show(&self) {
        match self{
            BaseElem::BlockElem(e) => {
                e.show();
            }
            BaseElem::UnKnownElem(e) => {
                e.show();
            }
        }
    }
}

trait ASTBranch{
    fn show(&self);
}


struct BlockBranch{
    undec_contents: Option<String>,
    contents: Option<Box<BaseElem>>
}

impl ASTBranch for BlockBranch{
    fn show(&self){
        match &self.undec_contents{
            Some(e) => {
                println!("undec_contents :{}", e);
            }
            None=>{}
        }
    }
}

struct UnKnownBranch{
    contents: char
}

impl ASTBranch for UnKnownBranch{
    fn show(&self){
        println!("{}", self.contents);
    }
}
struct Parser{
    code:String
}

impl Parser{
    fn new(code:String) -> Self{
        Self {
            code: code
        }
    }

    fn code2vec(&self,code: &str) -> Vec<BaseElem>{
        todo!();
    }


    fn code2_vec_pre_proc_func(&self, code:&String) -> Vec<BaseElem>{
        let mut rlist :Vec<BaseElem>= Vec::new();
        return code
                    .chars()
                    .map(|c|BaseElem::UnKnownElem(UnKnownBranch{contents: c}))
                    .collect();
    }

    fn grouping_block(&self,codelist: Vec<BaseElem>) -> Result<Vec<BaseElem>,&str>{
        let mut rlist:Vec<BaseElem> = Vec::new();
        let mut group:String = String::new();
        let mut depth:isize = 0;

        for inner in codelist{
            match inner{
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
                                BaseElem::BlockElem(
                                    BlockBranch{
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

// test case
#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn test00() {
        let program = String::from("{a{123}42{hello}}world");
        let parser = Parser::new(program);

        let rlst = parser.grouping_block(
            parser.code2_vec_pre_proc_func(&parser.code)
        ).unwrap();

        for i in rlst{
            i.show();
        }
        //println!("{:?}",rlst);
        assert_eq!(2 + 2, 4);
    }
}

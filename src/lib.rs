use std::iter::Enumerate;


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
    // input
    code:String,
    depth:usize,
    loopdepth:usize,
    // default settings
    left_priority_list:OpePairList,
    right_priority_list:OpePairList,
    prefix_priority_list:OpePairList,
}

impl Parser{

    fn new(code:String, depth:usize, loopdepth:i32) -> Self{
        // # left_priority_list initialize
        let mut left_priority_list:OpePairList = OpePairList::new(vec![
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
        let mut right_priority_list:OpePairList = OpePairList::new(vec![
            ("=", -4) ,
            ("+=", -4),
            ("-=", -4),
            ("*=", -4),
            ("/=", -4),
            // ## 二乗
            ("**", 3) ,
        ]);

        // # prefix_priority_list
        let mut prefix_priority_list:OpePairList = OpePairList::new(vec![
            ("!", -1),
        ]);

        return Self {
            code : code.clone(),
            depth: depth,
            loopdepth: loopdepth,
            left_priority_list: left_priority_list,
            right_priority_list: right_priority_list,
            prefix_priority_list:prefix_priority_list
        };
    }

	
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

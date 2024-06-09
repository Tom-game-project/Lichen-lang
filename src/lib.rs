mod core;
use core::*;

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

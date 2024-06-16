mod core;
use core::*;

// test case
#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn test00() {
        let program = String::from("{a{123\"456\"}\"42\"{hello}}world");
        let parser = Parser::new(
            program.clone(),
            0
        );

        println!("test case {}",program);
        let rlst = parser.resolve();

        match rlst {
            Ok(v) => 
            {
                for i in v{
                    i.show();
                }
            }
            Err(e) => 
            {
                println!("error msg {}", e);
            }
        }
        
        //println!("{:?}",rlst);
        //assert_eq!(2 + 2, 4);
    }
}

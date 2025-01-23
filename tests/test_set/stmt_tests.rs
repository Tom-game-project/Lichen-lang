// 文のテスト
// stmt

extern crate lichen_lang;
use lichen_lang::abs::ast::*;
use lichen_lang::parser::core_parser::Parser;
use lichen_lang::parser::stmt_parser::StmtParser;

/// 動かし方
/// ```bash
/// cargo test --package lichen-lang --test lib -- test_set::stmt_tests::stmt_test00 --exact --show-output
/// ```
#[test]
pub fn stmt_test00() {
    let mut ans_ast_string = String::new();
    let test_cases = vec![
        "
            let mut a = 1 + 1;
            let b = 1 + 1;
            if (a < b) // hello world
            {
                print(a, b);
            };
            b = 1 + 1;
        ",
        "
            let mut a = 1 + 1;
            let b = 1 + 1;
            if (a < b){
                print(a, b);
            }
            elif (a < b){
                print(a, b);
            };
            b = 1 + 1;
        ",
        "
        let a = tarai(tarai(x - 1,  y, z), tarai(y - 1, z, x), tarai(z - 1, x, y));
        ",
        "let a: i32 = 1+2;",
        "let a: (i32, i32) -> i32 -> i32 = f();",
        "
            if (a < b){
                gcd(a, b);
                gcd(if (a){gcd(a, b);} , b);
                gcd(a, b);
            }
            elif (a < b){
                gcd(a, b);
            };
            b = 1 + 1;
        ",
        "
        let a = 42;
        return a + 1;
        ",
        "
        a = 42;
        return a + 1;
        ",
        "
        a = 42;
        return (a + 1);
        ",
        "
        while (true){
            a += 1;
            if (a >= 5){
                print(\"hello\");
                break ;
            };
        } else {
            print(\"end\");
        }
        ",
        "
        let a:i32 = 42;
        let b:i32 = 100;
        let b:Vec(i32) = [1 , 2 , 3];
        let b:Vec(Vec(i32)) = [[1]];
        let func00:(i32, i32) -> (f32, f32) -> i32 = f;
        func00((a + b) / 2, 1);
        "
    ];

    for test_case in test_cases {
        let mut s_parser = StmtParser::new(test_case.to_string(), 0, 0);
        println!("----------------------------------------------------------------");
        if let Err(e) = s_parser.resolve() {
            println!("unexpected ParseError occured");
            println!("{:?}", e);
            panic!()
        } else {
            for i in s_parser.code_list {
                // 分けることのできない式の集合
                println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
                ans_ast_string.push_str(i.get_show_as_string().as_str());
            }
            println!("{}", ans_ast_string);
            ans_ast_string.clear();
        }
    }
}

/// 動かし方
///
/// ```bash
/// cargo test --package lichen-lang --test lib -- test_set::stmt_tests::stmt_test01 --exact --show-output
/// ```
#[test]
pub fn stmt_test01() {
    let mut ans_ast_string = String::new();
    let test_cases = vec![
        "
            let mut a = 1 + 1;
            let b = 1 + 1;
            if (a < b) // hello world
            {
                print(a, b);
            };
            b = 1 + 1;
        ",
        "
            let mut a = 1 + 1;
            let b = 1 + 1;
            if (a < b){
                print(a, b);
            }
            elif (a < b){
                print(a, b);
            };
            b = 1 + 1;
        ",
        "
        let a = tarai(tarai(x - 1,  y, z), tarai(y - 1, z, x), tarai(z - 1, x, y));
        ",
        "let a: i32 = 1+2;",
        "let a: (i32, i32) -> i32 -> i32 = f();",
        "
            if (a < b){
                gcd(a, b);
                gcd(if (a){gcd(a, b);} , b);
                gcd(a, b);
            }
            elif (a < b){
                gcd(a, b);
            };
            b = 1 + 1;
        ",
        "
        // こんにちは
        let i:i32 = 2;
        while (i < 100)
        {
            j = 2;
            c = 0;
            while (j < i) {
                if (i%j == 0) {
                    c = c + 1;
                };
                j = j + 1;
            };
            if (c == 0) {
                // is prime
                log(i);
            };
            i = i + 1;
        };
        ",
        "
        let a:i32 = 42;
        ",
        "
        let a:i32 ;
        "
    ];

    for test_case in test_cases {
        let mut s_parser = StmtParser::new(test_case.to_string(), 0, 0);
        println!("----------------------------------------------------------------");
        if let Err(e) = s_parser.resolve() {
            println!("unexpected ParseError occured");
            println!("{:?}", e);
            panic!()
        } else {
            for i in s_parser.code_list {
                // 分けることのできない式の集合
                println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
                println!("{:?}", i);
                ans_ast_string.push_str(i.get_show_as_string().as_str());
            }
            println!("{}", ans_ast_string);
            ans_ast_string.clear();
        }
    }
}

#[test]
pub fn stmt_test02() {
    let mut ans_ast_string = String::new();
    let test_cases = vec![
        "
        __mem[0][0] = 10;
        __mem[0] = 20;
        log(__mem[0] + __mem[1]);
        ",
    ];

    for test_case in test_cases {
        let mut s_parser = StmtParser::new(test_case.to_string(), 0, 0);
        println!("----------------------------------------------------------------");
        if let Err(e) = s_parser.resolve() {
            println!("unexpected ParseError occured");
            println!("{:?}", e);
            panic!()
        } else {
            for i in &s_parser.code_list {
                // 分けることのできない式の集合
                ans_ast_string.push_str(i.get_show_as_string().as_str());
            }
            println!("{}", ans_ast_string);
            ans_ast_string.clear();
            println!("{:?}", &s_parser.code_list);
        }
    }
}

#[test]
pub fn stmt_test03() {
    let test_cases = ["
        fn main(a:i32, b:i32) -> (i32, i32) -> i32 {
            let a:i32 = 1;
            let b:Vec[Vec[(i32, i32) -> i32]] = [[f]];
            print(\"hello world\");
            return (3);
        }
        "];

    for test_case in test_cases {
        let mut s_parser = StmtParser::new(test_case.to_string(), 0, 0);
        println!("----------------------------------------------------------------");
        if let Err(e) = s_parser.resolve() {
            println!("unexpected ParseError occured");
            println!("{:?}", e);
        } else {
            let mut a = String::default();

            for i in s_parser.code_list {
                a.push_str(&i.get_show_as_string());
            }

            println!("{}", a);
        }
    }
}

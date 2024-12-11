// 生成テスト
// 生成されたアセンブリが正しいかチェックする。
//

extern crate lichen_lang;
use colored::{Color, Colorize};
use lichen_lang::abs::ast::*;
use lichen_lang::abs::gen::Wasm_gen;
use lichen_lang::parser::expr_parser::ExprParser;
use lichen_lang::parser::{core_parser::Parser, stmt_parser::StmtParser};

// install wasmer
use wasmer::{imports, Instance, Module, Store, Value};

/// `a:i32` `b:i32`の２つの引数をうけとり一つ返り値を返却するような式をテストする
///
pub fn wasm_test_2_args_1_return(
    wasm_code: &str,
    test_args_set: &[(i32, i32)],
    ans: &[i32],
) -> anyhow::Result<()> {
    let module_wat = &format!(
        r#"
(module
(type $t0 (func
(param i32)
(param i32)
(result i32)
))
(func $test (export "test") (type $t0)
(param $a i32)
(param $b i32)
(result i32)
;; -- start --
{}
;; --  end  --
))
    "#,
        wasm_code
    );
    println!("--- wasm code ---");
    println!("{}", module_wat);

    let mut store = Store::default();
    let module = Module::new(&store, module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let wasm_test = instance.exports.get_function("test")?;

    for (i, &(arg1, arg2)) in test_args_set.iter().enumerate() {
        let result = wasm_test.call(&mut store, &[Value::I32(arg1), Value::I32(arg2)])?;
        println!(
            "a:{}, b:{}, result: {} ans: {} -> {}",
            arg1,
            arg2,
            result[0],
            ans[i],
            if result[0] == Value::I32(ans[i]) {
                "Ok".color(Color::Green)
            } else {
                "Failed".color(Color::Red)
            }
        );
        assert_eq!(result[0], Value::I32(ans[i]));
    }
    Ok(())
}

/// https://crates.io/crates/wasmer/
/// wasmerの使い方の例
#[test]
pub fn run_wasm() -> anyhow::Result<()> {
    let module_wat = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
    (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        local.get $p0
        i32.const 1
        i32.add
    ))
    "#;

    let mut store = Store::default();
    let module = Module::new(&store, module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let add_one = instance.exports.get_function("add_one")?;

    for i in 0..10 {
        let result = add_one.call(&mut store, &[Value::I32(i)])?;
        println!("i:{} result: {}", i, result[0]);
        // assert_eq!(result[0], Value::I32(i + 1));
    }
    Ok(())
}

/// simdの実験
#[test]
pub fn run_wasm2() -> anyhow::Result<()> {
    let module_wat = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
    (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
    (local $a v128)
    v128.const i32x4 3 2 3 4  ;; v128(3, 2, 3, 4)
    local.set $a
    local.get $a
    i32x4.extract_lane 3
    ))
    "#;

    let mut store = Store::default();
    let module = Module::new(&store, module_wat)?;
    // The module doesn't import anything, so we create an empty import object.

    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;

    for i in 0..10 {
        let result = add_one.call(&mut store, &[Value::I32(i)])?;
        println!("i:{} result: {}", i, result[0]);
        // assert_eq!(result[0], Value::I32(i + 1));
    }
    Ok(())
}

/// wasmでループを作る実験
#[test]
pub fn run_wasm3() -> anyhow::Result<()> {
    let module_wat = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
    (func $while_test (export "while_test") (type $t0) (param $p0 i32) (result i32)
    (local $i i32)
    i32.const 0
    local.set $i
    ;;
    loop $0
    block $1
        i32.const 10
        local.get $i
        i32.eq
        br_if $1
        i32.const 1
        local.get $i
        i32.add
        local.set $i
        br $0
    end
    end
    local.get $i
    ))
    "#;

    let mut store = Store::default();
    let module = Module::new(&store, module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("while_test")?;

    for i in 0..10 {
        let result = add_one.call(&mut store, &[Value::I32(i)])?;
        println!("i:{} result: {}", i, result[0]);
        // assert_eq!(result[0], Value::I32(i + 1));
    }
    Ok(())
}

///
/// malloc, free実装に向けての準備
#[test]
pub fn run_wasm4() -> anyhow::Result<()> {
    let module_wat = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
      (memory $memory 1)
      (export "memory" (memory $memory))

    (func $while_test (export "while_test") (type $t0) (param $p0 i32) (result i32)
    (local $i i32)
    i32.const 0
    local.set $i
    ;;
    loop $0
    block $1
        i32.const 10
        local.get $i
        i32.eq
        br_if $1
        i32.const 1
        local.get $i
        i32.add
        local.set $i
        br $0
    end
    end
    local.get $i
    ))
    "#;

    let mut store = Store::default();
    let module = Module::new(&store, module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("while_test")?;

    for i in 0..10 {
        let result = add_one.call(&mut store, &[Value::I32(i)])?;
        println!("i:{} result: {}", i, result[0]);
        // assert_eq!(result[0], Value::I32(i + 1));
    }
    Ok(())
}

#[test]
pub fn gen_test00() {
    let test_cases = vec![
        "!a&&!b",
        "a != b",
        "0 <= a && a <= 10",
        "-10+20",
        // "a**b**c" ,
        "a+b+c",
        "(a+bc)+(cde-defg)",
        "func(10,1)+2*x",
        // "tarai(1)(2)(3)"    ,
        // "if (0 < x){ 1 } else {0} + 1" ,
        "c = !a&&!b",
        "d = -10+20",
        // "a**b**c" ,
        "e = a+b+c",
        "f = (a+bc)+(cde-defg)",
        "g = func(10,1)+2*x",
        "var = (10 - 1) + 2 * ((1 + 4) * 5)", // 59
        "tarai(tarai(x - 1,  y, z), tarai(y - 1, z, x), tarai(z - 1, x, y))",
    ];

    for code in test_cases {
        let string_code: String = String::from(code);
        let mut e_parser = ExprParser::new(string_code, 0, 0);
        println!("------------------------------------------");
        println!("test case -> {}", code);
        match e_parser.resolve() {
            Err(e) => println!("{:?}", e),
            Ok(_) => {
                for i in e_parser.code_list {
                    i.show();
                    println!("--- wasm code ---");
                    match &i {
                        ExprElem::FuncElem(a) => {
                            match a.generate_wasm() {
                                Ok(wasm_code) => {
                                    //
                                    println!("{}", wasm_code);
                                }
                                Err(e) => {
                                    println!("wasm生成中にerrorが発生しました");
                                    println!("{:?}", e);
                                }
                            }
                        }
                        _ => {
                            println!("func 要素ではありませんでした");
                            continue;
                        }
                    }
                }
            }
        }
    }
}

///
///
#[test]
pub fn gen_test01() {
    let test_cases = [
        "!a&&!b",
        "!(a||b)",
        "a != b",
        "a * b != a + b",
        "-a <= b && b <= a",
    ];

    let arg_set: Vec<Vec<(i32, i32)>> = vec![
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // !a&&!b
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // !(a||b)
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // a != b
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // a * b != a + b
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // -a <= b && b <= a
    ];

    let ans: Vec<Vec<i32>> = vec![
        vec![1, 0, 0, 0], // !a&&!b
        vec![1, 0, 0, 0], // !(a||b)
        vec![0, 1, 1, 0], // a != b
        vec![0, 1, 1, 1], // a * b != a + b
        vec![1, 0, 1, 1], // -a <= b && b <= a
    ];

    for (i, &code) in test_cases.iter().enumerate() {
        let string_code: String = String::from(code);
        let mut e_parser = ExprParser::new(string_code, 0, 0);
        println!("------------------------------------------");
        println!("test case -> {}", code);
        match e_parser.resolve() {
            Err(e) => println!("{:?}", e),
            Ok(_) => {
                for expr in e_parser.code_list {
                    expr.show();
                    match &expr {
                        ExprElem::FuncElem(a) => match a.generate_wasm() {
                            Ok(wasm_code) => {
                                let _ = wasm_test_2_args_1_return(&wasm_code, &arg_set[i], &ans[i]);
                            }
                            Err(e) => {
                                println!("wasm生成中にerrorが発生しました");
                                println!("コンパイラに何らかの問題があります");
                                println!("{:?}", e);
                            }
                        },
                        _ => {
                            println!("func 要素ではありませんでした");
                            continue;
                        }
                    }
                }
            }
        }
    }
}

#[test]
pub fn gen_test02() {
    let test_cases = [
        "
        i = 0;
        while (i < 10)
        {
            log(i);
            i = i + 1;
        };
        ",
        "
        i = 0;
        while (i < 10)
        {
            j = 0;
            while (!!(j < 10) && -1 < 0) {
                /*hello world*/
                log(i);
                log(j);
                j = j + 1;
            };
            i = i + 1;
        };
        ",
        "
        i = 0;
        while (i < 10)
        {
            log(0);
            i = i + 1;
        };
        i = 0;
        while (i < 10)
        {
            log(1);
            i = i + 1;
        };
        ",
        "
        i = 0;
        while (i < 10)
        {
            log(i);
            if (5 <= i) {
                break;
            };
            i = i + 1;
        };
        ",
        "
        i = 0;
        while (i < 10)
        {
            j = 0;
            while (!!(j < 10) && -1 < 0) {
                /*hello world*/
                log(i); // hello
                log(j);
                break;
                j = j + 1;
            };
            i = i + 1;
        };
        ",
        "
        // 100までの素数を求めるプログラム
        i = 2;
        while (i < 100)
        {
            //コメント
            j = 2;
            c = 0;
            while (j < i) {
            // コメント２
                if (i%j == 0) {
                    // コメント ブロック内で更にコメントをする場合
                    c += 1;
                };
                j += 1;
            };
            if (c == 0) {
                // is prime
                log(i);
            };
            i += 1;
        };
        ",
        "
        // 100までの素数を求めるプログラム
        i = 2;
        counter = 0;
        while (i < 100)
        {
            //コメント
            j = 2;
            c = 0;
            while (j < i) {
            // コメント２
                if (i%j == 0) {
                    // コメント ブロック内で更にコメントをする場合
                    c += 1;
                };
                j += 1;
            };
            if (c == 0) {
                // is prime
                __mem[counter * 4] = i;
                counter += 1;
            };
            i +=1;
        };
        ",
        "
        __mem[a] = 42;
        log(__mem[a]);
        ",
        "
        i = 0;
        while (i < 10)
        {
            __mem[i * 4] = -1;
            log(__mem[i * 4]);
            i += 1;
        };
        ",
    ];

    for test_case in test_cases {
        let mut s_parser = StmtParser::new(test_case.to_string(), 0, 0);
        println!("----------------------------------------------------------------");
        if let Err(e) = s_parser.resolve() {
            println!("unexpected ParseError occured");
            println!("{:?}", e);
        } else {
            let mut wasm_text_format = String::new();

            for inner in s_parser.code_list {
                // 分けることのできない式の集合
                if let StmtElem::ExprElem(expr_b) = &inner{
                    if let Ok(a) = &expr_b.generate_wasm() {
                        wasm_text_format.push_str(a);
                    } else if let Err(e) = &expr_b.generate_wasm() {
                        println!("wasm生成中にエラーが発生しました(ExprElem)");
                        println!("{:?}", e);
                        panic!();
                    }
                } else if let StmtElem::Special(controll_b) = &inner {
                        if let Ok(a) = &controll_b.generate_wasm() {
                            wasm_text_format.push_str(a);
                        } else if let Err(e) = &controll_b.generate_wasm() {
                            println!("wasm生成中にエラーが発生しました(ControlElem)");
                            println!("{:?}", e);
                            panic!();
                        }

                } else if let StmtElem::CommentElem(_) = &inner {
                    // pass
                    // 文中にコメントが入った場合はpass
                } else {
                    panic!()
                }
            }
            println!("{}", wasm_text_format);
        }
    }
}


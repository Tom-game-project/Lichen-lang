use crate::abs::ast::{ExprElem, StmtElem};
use crate::abs::gen::Wasm_gen;
use crate::errors::generate_errors::GenerateError;

use crate::token::func::FuncBranch;
use crate::token::item::ItemBranch;
use crate::token::list::ListBranch;
use crate::token::operator::OperatorBranch;
use crate::token::paren_block::ParenBlockBranch;
use crate::token::syntax::SyntaxBranch;
use crate::token::syntax_box::SyntaxBoxBranch;

pub const LOOP_ADDR: &str = "#l";
pub const BLOCK_ADDR: &str = "#b";
// memoryにアクセスするための特別な配列の名前
pub const MEMORY_SPACE_NAME: &str = "__mem";

/// function branch
impl Wasm_gen for FuncBranch {
    fn generate_wasm(&self) -> Result<String, GenerateError> {
        let mut assembly_text: String = String::new();

        // 関数処理部分
        match &*self.name {
            // pass
            ExprElem::OpeElem(ope_b) => {
                // 演算子のとき
                // 必ず２つの引数が渡されるが`-1`などの場合に注意が必要
                assembly_text.push_str(&ope_b.generate_wasm(&self.contents[0], &self.contents[1])?);
            }

            ExprElem::WordElem(word_b) => {
                // 普通の関数のとき
                // 引数処理部分
                for i in &self.contents {
                    match i {
                        ExprElem::ItemElem(b) => {
                            assembly_text.push_str(&b.generate_wasm()?);
                        }
                        _ => {
                            // 必ず引数はアイテムになるのでエラー
                            // ここにitem以外の要素を検知した場合は、
                            // コンパイラの実装に何らかの問題があります
                            return Err(GenerateError::Deverror);
                        }
                    }
                }
                assembly_text.push_str(&format!("call ${}\n", word_b.contents));
            }

            _ => {
                // ここは関数を返すifやwhileを定義しない限りerrorになる
                return Err(GenerateError::Deverror);
            }
        }
        Ok(assembly_text)
    }
}

impl Wasm_gen for ItemBranch {
    fn generate_wasm(&self) -> Result<String, GenerateError> {
        // 複数の場合もあることに注意
        let mut assembly_text = String::default();
        if self.contents.is_empty() {
            // itemの中に何も要素を持たない場合
            // 例えば、考えられるのは'-'だったりする場合
        } else if self.contents.len() == 1 {
            // Itemの中に要素が一つだけの場合
            // （特別に修飾子が付与されない場合）
            // TODO
            // ここでは、変数をi32として扱います
            match &self.contents[0] {
                ExprElem::WordElem(word_b) => {
                    if word_b.self_is_num()? {
                        // もし数字だった場合
                        assembly_text.push_str(&format!("i32.const {}\n", word_b.contents));
                    } else {
                        // もし何らかの変数だった場合
                        assembly_text.push_str(&format!("local.get ${}\n", word_b.contents));
                    }
                }

                ExprElem::FuncElem(func_b) => {
                    assembly_text.push_str(&func_b.generate_wasm()?);
                }

                ExprElem::ParenBlockElem(paren_b) => {
                    assembly_text.push_str(&paren_b.generate_wasm()?);
                }
                ExprElem::ListElem(list_b) => {
                    assembly_text.push_str(&list_b.generate_name_wasm()?);
                    assembly_text.push_str("i32.load\n");
                }
                _ => {
                    return Err(GenerateError::Deverror);
                }
            }
        } else {
            // ここは例えば、let mut aなどの場合
            //
            // `borrow mut` `&mut` とか引数に
            return Err(GenerateError::Deverror); // 未実装
        }
        Ok(assembly_text)
    }
}

impl ListBranch {
    /// indexの展開
    pub fn generate_contents_wasm(&self) -> Result<String, GenerateError> {
        let mut assembly_text = String::default();
        match &self.contents[0] {
            ExprElem::ItemElem(item_b) => {
                assembly_text.push_str(&item_b.generate_wasm()?);
            }
            _ => {
                return Err(GenerateError::Deverror);
            }
        }
        Ok(assembly_text)
    }

    pub fn generate_name_wasm(&self) -> Result<String, GenerateError> {
        let mut assembly_text = String::default();
        if let ExprElem::WordElem(word_b) = &*self.name {
            if word_b.contents == MEMORY_SPACE_NAME {
                // 特別なケース、メモリに直接アクセスするための方法を提供する
                // ```
                // __mem[0] = 0;
                // ```
                assembly_text.push_str(&self.generate_contents_wasm()?);
            } else {
                // 通常のケース
                // ```
                // a[0] = 0;
                // ```
                todo!()
            }
        } else {
            // 呼び出しの対象が名前ではない場合
            // ```
            // lst[0][0]
            // ^^^^^^
            // ```
            todo!()
        }
        Ok(assembly_text)
    }
}

// Wasm_gen ---

/// wasmで、メモリから値を取得する命令を記述する
pub fn wasm_get_memory_value_gen(index: i32) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();

    assembly_text.push_str(&format!("i32.const {}", index));
    assembly_text.push_str("i32.load\n");

    Ok(assembly_text)
}

///  wasmで、メモリに値を書く命令を記述する
pub fn wasm_set_memory_value_gen(
    index: i32,
    r_assembly_text: &str,
) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();

    assembly_text.push_str(&format!("i32.const {}", index));
    assembly_text.push_str(r_assembly_text);
    assembly_text.push_str("i32.store\n");
    Ok(assembly_text)
}

impl OperatorBranch {
    pub fn generate_wasm(
        &self,
        l_expr: &ExprElem,
        r_expr: &ExprElem,
    ) -> Result<String, GenerateError> {
        // 最初はi32向のみ対応
        // ここで送られて来るデータが本当に文字列でいいのか考える
        let mut assembly_text = String::default();
        match &*self.ope {
            "=" => assembly_text.push_str(&equal_gen_wasm(l_expr, r_expr)?), // equal
            "+=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "+=")?),
            "-=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "-=")?),
            "*=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "*=")?),
            "/=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "/=")?),
            "%=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "%=")?),
            "+" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.add\n")?),
            "-" => assembly_text.push_str(&sub_gen_wasm(l_expr, r_expr)?), // subtract
            "*" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.mul\n")?),
            "/" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.div\n")?),
            "%" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.rem_s\n")?),
            "&&" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.and\n")?),
            "||" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.or\n")?),
            "!" => assembly_text.push_str(&not_gen_wasm(l_expr, r_expr)?), // xor を使ってnotを再現している
            //
            "==" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.eq\n")?),
            "!=" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.ne\n")?),
            // 大小には`signed` `unsigned`がある
            "<" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.lt_s\n")?),
            ">" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.gt_s\n")?),
            "<=" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.le_s\n")?),
            ">=" => assembly_text.push_str(&normal_ope_gen_wasm(l_expr, r_expr, "i32.ge_s\n")?),
            _ => return Err(GenerateError::InvalidOperation),
        }
        Ok(assembly_text)
    }
}

fn equal_gen_wasm(l_expr: &ExprElem, r_expr: &ExprElem) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    let mut r_assembly_text = String::default();

    if let ExprElem::ItemElem(item_b) = r_expr {
        r_assembly_text = item_b.generate_wasm()?;
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = l_expr {
        // とりあえず、パターンなどを考えず、一つの変数に値を代入する
        // 場合のみの実装
        if let ExprElem::WordElem(word_b) = &item_b.contents[0] {
            // 普通の変数に代入するのと同じ
            // a = 1;
            // のようなケース
            assembly_text.push_str(&r_assembly_text);
            assembly_text.push_str(&format!("local.set ${}\n", word_b.contents));
        } else if let ExprElem::ListElem(list_b) = &item_b.contents[0] {
            // pass
            // TODO
            // a[0] = 1;のようなケース
            // ^
            // 呼び出す対象が名前の場合
            // ```
            // <list elem> = <r_expr>
            // ```
            //
            assembly_text.push_str(&list_b.generate_name_wasm()?);
            assembly_text.push_str(&r_assembly_text);
            assembly_text.push_str("i32.store\n");
        } else {
            // word 以外がパターンに渡された場合
            return Err(GenerateError::InvalidleftPattern);
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    Ok(assembly_text)
}

pub fn ref_aequal_gen_wasm(
    l_expr: &ExprElem,
    r_expr: &ExprElem,
    ope: &str,
) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    let r_assembly_text: String;
    let getter_assembly_text: String;
    let setter_assembly_text: String;

    // a += 1;
    // ^    ^
    // a = a + 1;
    if let ExprElem::ItemElem(item_b) = r_expr {
        r_assembly_text = item_b.generate_wasm()?;
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = l_expr {
        // 左は式ではなくパターンの処理をする必要があります
        if let ExprElem::WordElem(word_b) = &item_b.contents[0] {
            // pass
            setter_assembly_text = format!("local.set ${}\n", word_b.contents); // setter
            getter_assembly_text = format!("local.get ${}\n", word_b.contents); // setter
        } else {
            println!("まだサポートしていない書き方です"); // TODO
            todo!()
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    // ```wat
    // ;; 10 - 3
    // local.get $a
    // i32.const 1
    // i32.sub ;; sub
    // local.set $a
    // ```
    assembly_text.push_str(&getter_assembly_text);
    assembly_text.push_str(&r_assembly_text);
    match ope {
        "+=" => assembly_text.push_str("i32.add\n"),
        "-=" => assembly_text.push_str("i32.sub\n"),
        "*=" => assembly_text.push_str("i32.mul\n"),
        "/=" => assembly_text.push_str("i32.div\n"),
        "%=" => assembly_text.push_str("i32.rem_s\n"),
        _ => return Err(GenerateError::Deverror),
    }
    assembly_text.push_str(&setter_assembly_text);
    Ok(assembly_text)
}

/// ふたつの引数を両端からとる"普通の"演算子の生成
fn normal_ope_gen_wasm(
    l_expr: &ExprElem,
    r_expr: &ExprElem,
    ope_string: &str,
) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = l_expr {
        assembly_text.push_str(&item_b.generate_wasm()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = r_expr {
        assembly_text.push_str(&item_b.generate_wasm()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    assembly_text.push_str(ope_string);
    Ok(assembly_text)
}

/// 前置記法の場合わけが必要なケース("-"の場合)
fn sub_gen_wasm(l_expr: &ExprElem, r_expr: &ExprElem) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = l_expr {
        if item_b.has_no_elem() {
            assembly_text.push_str("i32.const 0\n");
        } else {
            assembly_text.push_str(&item_b.generate_wasm()?);
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = r_expr {
        assembly_text.push_str(&item_b.generate_wasm()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    assembly_text.push_str("i32.sub\n");
    Ok(assembly_text)
}

/// 前置記法の場合わけが必要なケース("!"の場合)
fn not_gen_wasm(l_expr: &ExprElem, r_expr: &ExprElem) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = l_expr {
        if item_b.has_no_elem() {
            assembly_text.push_str("i32.const 1\n");
        } else {
            assembly_text.push_str(&item_b.generate_wasm()?);
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = r_expr {
        assembly_text.push_str(&item_b.generate_wasm()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    assembly_text.push_str("i32.xor\n");
    Ok(assembly_text)
}

impl Wasm_gen for ParenBlockBranch {
    fn generate_wasm(&self) -> Result<String, crate::errors::generate_errors::GenerateError> {
        let mut assembly_text = String::default();
        match self.contents.len() {
            0 => {
                // pass
                // カッコの中に何もない場合
                // ()
                // ただ、ここではは想定されていないエラーをキャッチする必要がある
                // ```
                // (()) / a
                // ```
            }

            1 => {
                match &self.contents[0] {
                    // example `(1+a)`
                    ExprElem::FuncElem(func_b) => {
                        assembly_text.push_str(&func_b.generate_wasm()?);
                    }

                    // example `(a)`
                    ExprElem::WordElem(word_b) => {
                        if word_b.self_is_num()? {
                            assembly_text.push_str(&format!("i32.const {}\n", word_b.contents));
                        } else {
                            // もし何らかの変数だった場合
                            assembly_text.push_str(&format!("local.get ${}\n", word_b.contents));
                        }
                    }

                    // `((a + 1))`
                    ExprElem::ParenBlockElem(paren_b) => {
                        assembly_text.push_str(&paren_b.generate_wasm()?);
                    }

                    _ => {
                        // pass
                        return Err(GenerateError::Deverror);
                    }
                }
            }
            _ => {
                // 対応していない
                // `(,,,)`的なシチュエーション
                return Err(GenerateError::Deverror);
            }
        }
        Ok(assembly_text)
    }
}

impl Wasm_gen for SyntaxBoxBranch {
    fn generate_wasm(&self) -> Result<String, GenerateError> {
        let mut assembly_text = String::default();
        match &*self.name {
            "if" => {
                for section in &self.contents {
                    assembly_text.push_str(&section.generate_wasm("if")?);
                }
                for i in 0..count_if_section(&self.contents) {
                    assembly_text.push_str("end\n");
                }
            }
            "while" => {
                for section in &self.contents {
                    assembly_text.push_str(&section.generate_wasm("while")?);
                }
            }
            "for" => {
                todo!()
            }
            _ => {
                return Err(GenerateError::Deverror);
            }
        }
        Ok(assembly_text)
    }
}

fn count_if_section(if_state_contents: &[SyntaxBranch]) -> usize {
    let mut c = 0;
    for inner in if_state_contents {
        match &*inner.name {
            "if" | "elif" => {
                c += 1;
            }
            _ => {
                // pass
            }
        }
    }
    c
}

impl SyntaxBranch {
    pub fn generate_wasm(&self, head_name: &str) -> Result<String, GenerateError> {
        let mut assembly_text = String::new();
        match head_name {
            "if" => {
                assembly_text.push_str(&wasm_if_gen(self)?);
            }
            "while" => {
                assembly_text.push_str(&wasm_while_gen(self)?);
            }
            "for" => {
                todo!()
            }
            _ => {
                return Err(GenerateError::Deverror);
            }
        }
        Ok(assembly_text)
    }
}

fn wasm_if_gen(if_state: &SyntaxBranch) -> Result<String, GenerateError> {
    let mut assembly_text = String::new();

    match &*if_state.name {
        "if" => {
            if if_state.expr.is_empty() {
                // TODO
                // if の条件式が空はおかしいというerror
                // を返す
                todo!()
            } else if if_state.expr.len() == 1 {
                if let ExprElem::FuncElem(func_b) = &if_state.expr[0] {
                    // 式を展開
                    assembly_text.push_str(&func_b.generate_wasm()?);
                    assembly_text.push_str("if\n");
                    // 文をwasmように展開
                    assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
                } else if let ExprElem::ParenBlockElem(paren_b) = &if_state.expr[0] {
                    // 式を展開
                    assembly_text.push_str(&paren_b.generate_wasm()?);
                    assembly_text.push_str("if\n");
                    // 文をwasmように展開
                    assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
                } else {
                    // TODO
                    // if の条件式が関数ではないのはおかしいというerror
                    // を返す
                    todo!()
                }
            } else {
                // ここで、exprelemが複数あるのはおかしい
                return Err(GenerateError::Deverror);
            }
        }
        "elif" => {
            if if_state.expr.is_empty() {
                // TODO
                // else if の条件式が空はおかしいというerror
                // を返す
                todo!()
            } else if if_state.expr.len() == 1 {
                if let ExprElem::FuncElem(func_b) = &if_state.expr[0] {
                    // 式を展開
                    assembly_text.push_str("else\n");
                    assembly_text.push_str(&func_b.generate_wasm()?);
                    assembly_text.push_str("if\n");
                    // 文をwasmように展開
                    assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
                } else if let ExprElem::ParenBlockElem(paren_b) = &if_state.expr[0] {
                    // 式を展開
                    assembly_text.push_str("else\n");
                    assembly_text.push_str(&paren_b.generate_wasm()?);
                    assembly_text.push_str("if\n");
                    // 文をwasmように展開
                    assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
                } else {
                    // TODO
                    // else if の条件式が関数ではないのはおかしいというerror
                    // を返す
                    todo!()
                }
            } else {
                // ここで、exprelemが複数あるのはおかしい
                return Err(GenerateError::Deverror);
            }
        }
        "else" => {
            if if_state.expr.is_empty() {
                assembly_text.push_str("else\n");
                assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
            } else {
                // else 説に条件式を設定しているのはおかしいというerror
                todo!()
            }
        }
        _ => return Err(GenerateError::Deverror),
    }
    Ok(assembly_text)
}

fn wasm_while_gen(while_state: &SyntaxBranch) -> Result<String, GenerateError> {
    use crate::gen::wasm::{BLOCK_ADDR, LOOP_ADDR};
    let mut assembly_text = String::default();

    // pass
    match &*while_state.name {
        "while" => {
            let loop_addr = format!("{}{}", LOOP_ADDR, while_state.loopdepth);
            let block_addr = format!("{}{}", BLOCK_ADDR, while_state.loopdepth);

            assembly_text.push_str(&format!("loop ${}\n", loop_addr));
            assembly_text.push_str(&format!("block ${}\n", block_addr));
            if while_state.expr.is_empty() {
                // TODO
                // if の条件式が空はおかしいというerror
                // を返す
                todo!()
            } else if while_state.expr.len() == 1 {
                if let ExprElem::FuncElem(func_b) = &while_state.expr[0] {
                    // 式を展開
                    assembly_text.push_str(&func_b.generate_wasm()?);
                } else if let ExprElem::ParenBlockElem(paren_b) = &while_state.expr[0] {
                    // 式を展開
                    assembly_text.push_str(&paren_b.generate_wasm()?);
                } else {
                    // TODO
                    // if の条件式が関数またはカッコではないのはおかしいというerror
                    // を返す
                    todo!()
                }
            } else {
                // ここで、exprelemが複数あるのはおかしい
                return Err(GenerateError::Deverror);
            }
            // not
            assembly_text.push_str("i32.const 1\n");
            assembly_text.push_str("i32.xor\n");
            assembly_text.push_str(&format!("br_if ${}\n", block_addr));
            assembly_text.push_str(&wasm_stmt_gen(&while_state.contents)?);
            assembly_text.push_str(&format!("br ${}\n", loop_addr));
            assembly_text.push_str("end\n");
            assembly_text.push_str("end\n");
        }
        _ => {
            // dev error
            return Err(GenerateError::Deverror);
        }
    }
    Ok(assembly_text)
}

fn wasm_stmt_gen(stmt_list: &[StmtElem]) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    for s in stmt_list {
        if let StmtElem::ExprElem(expr_b) = s {
            assembly_text.push_str(&expr_b.generate_wasm()?);
        } else if let StmtElem::Special(control_b) = s {
            assembly_text.push_str(&control_b.generate_wasm()?);
        } else if let StmtElem::CommentElem(comment_b) = s {
            // pass
            assembly_text.push_str("");
        } else {
            // これ以外のわたしが認識していない場合
            // コメントだった場合について実装する
            //
            todo!()
        }
    }
    Ok(assembly_text)
}

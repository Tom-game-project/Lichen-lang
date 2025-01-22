# tests

## テストすべき項目

- expr_prser

- state_parser

- ASTの構造チェック

### expr test 00
`test case`
```
 !a&& !b
```

```bash
cargo test --package lichen-lang --test lib -- test_set::expr_tests::expr_test00 --exact --show-output
```

### expr test 01
`test case`
```
func00(10, 123 + func01(a,b,c)) + 2 * x
```

```bash
cargo test --package lichen-lang --test lib -- test_set::expr_tests::expr_test01 --exact --show-output
```

### expr test 02
debug trait test
```
(10 + 1) + 2 * x
```

```bash
cargo test --package lichen-lang --test lib -- test_set::expr_tests::expr_test02 --exact --show-output
```

### expr test 03

```
while  (0 < x) { 1 } else {0}(1)[2](3)
```

```bash
cargo test --package lichen-lang --test lib -- test_set::expr_tests::expr_test03 --exact --show-output
```

### expr test04

```
/*hello world*/
```

```bash
cargo test --package lichen-lang --test lib -- test_set::expr_tests::expr_test04 --exact --show-output
```

### unit test00

expr_parserが正常に動作するかを確かめるテスト00
```bash
cargo test --package lichen-lang --test lib -- tests::unit_test00 --exact --show-output
```

### unit test01

expr_parserが正常に動作するかを確かめるテスト01
ここでは、`callable` `subscriptable`なコードが正常な動作をするかどうかのテストをします

```bash
cargo test --package lichen-lang --test lib -- tests::unit_test01 --exact --show-output
```

### stmt test00
```bash
cargo test --package lichen-lang --test lib -- test_set::stmt_tests::stmt_test00 --exact --show-output
```

### stmt test01
```bash
cargo test --package lichen-lang --test lib -- test_set::stmt_tests::stmt_test01 --exact --show-output
```

### stmt test02
```bash
cargo test --package lichen-lang --test lib -- test_set::stmt_tests::stmt_test02 --exact --show-output
```

### stmt test03
```bash
cargo test --package lichen-lang --test lib -- test_set::stmt_tests::stmt_test03 --exact --show-output
```

### type test00
```bash
cargo test --package lichen-lang --test lib -- test_set::type_tests::type_test00 --exact --show-output
```



### gen test00
正しくwasmが生成できるかのテスト
```
cargo test --package lichen-lang --test lib -- test_set::gen_tests::gen_test00 --exact --show-output
```

### gen test01
生成された`wasm text format` が期待する動作をするかどうか確かめるテスト
```
cargo test --package lichen-lang --test lib -- test_set::gen_tests::gen_test01 --exact --show-output
```

### gen test02
文のコンパイルができるかどうかを確かめる
```
cargo test --package lichen-lang --test lib -- test_set::gen_tests::gen_test02 --exact --show-output
```

## テストの走るタイミング

テストはローカル環境で上のコマンドで実行することができる。

developまたは,masterブランチへPRを送ったとき。また、それぞれにマージされたとき以下のテストが実行される。

```bash
cargo test
```

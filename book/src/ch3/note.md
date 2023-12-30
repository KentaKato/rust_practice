# 所有権システム

## 代入では所有権が移動する

## 例１

OK
```rust
fn main(){
    let g1 = String::from("sample");
    let g2 = g1;
    println!("{}", g2);
}
```

NG
- g1からg2に所有権が移っている
```rust
fn main(){
    let g1 = String::from("sample");
    let g2 = g1;
    println!("{}", g1);
}
```

## 例２

末尾のprintln!にて、s1やs2を表示しようとするとコンパイルエラーになる。
所有権がs1 -> s2に移ったあと、s2が破棄されているから。
```rust
fn main(){
    let s1 = String::from("sample1");
    let s3 = String::from("sample2");
    {
        let s2 = s1;
        println!("{}", s2);
    }
    // println!("{}", s1);
    // println!("{}", s2);
    println!("{}", s3);
}
```

## 例外

基本型（整数、浮動小数点数、真偽など）は代入でコピーされる。
この理由は、基本型はスタック領域に確保されるため複製が高速で容易だから。
一方、文字列や構造体などのサイズが可変なデータはヒープ領域に確保される。
任意のタイミングでメモリを確保・解放できるが遅い。
そのためデフォルトでコピーされず、所有権が移る。

OK
```rust
fn main(){
    let g1 = 30;
    let g2 = g1;
    println!("{}", g1);
}
```

## 関数の値渡しでは所有権が移る

NG
- show_messageが値渡しになっている。C++のようにコピーされず、所有権が渡される。そのため、g1が使えなくなる。

```rust
fn main(){
    let g1 = String::from("sample");
    show_message(g1);
    println!("{}", g1);
}

fn show_message(message: String){
    println!("{}", message);
}
```

OK
- show_messageは値渡しだが、Stringを返す。そのため、g1に所有権が戻されるためエラーにならない。
- g1をmutableで定義する必要があることに注意

```rust
fn main(){
    let mut g1 = String::from("sample");
    g1 = show_message(g1);
    println!("{}", g1);
}

fn show_message(message: String) -> String{
    println!("{}", message);
    return message;
}
```

### 参照と借用の活用
参照渡しにすれば、関数の呼び出し時に所有権が関数に貸し出され、関数を抜けるときに所有権が返却される。
このことから参照渡しを「借用」と呼ぶ。

OK
```rust
fn main(){
    let g1 = String::from("sample");
    show_message(&g1);
    println!("{}", g1);
}

fn show_message(message: &String) -> String{
    println!("{}", message);
}
```

## 関数から参照を返すときは生存期間に注意

NG
- 関数から参照を返している。しかし、関数を抜けるときに参照先のmsgが破棄されるためエラーになる。

```rust
fn gen_message() -> &str {
    let msg = String::from("sample");
    return &msg;
}

fn main(){
    let m = message();
    println!("{}", m);
}
```

## ミュータブルな参照

引数に`&mut`をつける。


OK
- 関数内で参照のまま扱えるケース
```rust
fn add_quate(msg: &mut String){
    msg.insert(0, 'S');
    msg.push('e');
}

fn main(){
    let mut msg = String::from("ampl");
    println!("{}", msg);
    add_quate(&mut msg);
    println!("{}", msg);
}
```

OK
- 関数内で、参照ではなく実際の値を扱うケース
- C++と異なり*が必要な点に注意

```rust
fn x2(arg: &mut i32){
    *arg = *arg * 2;
}

fn main(){
    let mut v = 16;
    x2(&mut v);
    println!("{}", v);
}

```


# シャドーイング

- 同一スコープ内で変数を再宣言できる機能。
- 再宣言すると、以前に宣言した変数を隠し、あとで宣言した変数が参照される。


危険な気がするけど、メリットのほうが大きいのか？

OK

```rust
fn main(){
    let s = "This is sample sentence.";
    let s = s.replace("This", "That");
    let s = s.replace("sentence", "code");
    println!("{}", s);
}
```

NG

```rust
fn main(){
    let s = "This is sample sentence.";
    let s = s.replace("This", "That");
    let s = 1;
    let s = 1.0;
    let s = s.replace("sentence", "code");
    println!("{}", s);
}
```

# テストフレームワーク

テストプロジェクトの作成

```bash
cargo new mytest --lib
cd mytest && tree .
.
├── Cargo.toml
└── src
    └── lib.rs
```

テストのテンプレートが自動生成される。

mytest/src/lib.rs
```rust
#[cfg(test)] // cargo testを実行したときに、ビルド対象となることを明示
             // #[...]はRustコンパイラーに与えるアトリビュートというメタ情報（？？？）

mod tests {  // モジュール'tests'を宣言
    #[test]  // テストコマンドの実行時、この宣言のある関数が実行される
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

ビルドしてテストしてみる
```bash
$ cargo test

   Compiling mytest v0.1.0 (/home/kato/rust_practice/src/ch3/mytest)
    Finished test [unoptimized + debuginfo] target(s) in 0.24s
     Running unittests src/lib.rs (target/debug/deps/mytest-79593358f0ff4f08)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mytest

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

マクロ一覧

assert!(var)
assert_eq!(var1, var2)
assert_ne!(var1, var2)


PartialEq

構造体同士の比較に必要。
冒頭で
#[derive(PartialEq)]
と宣言することで、assert_eq!(struct1, struct2)のように構造体同士の比較が可能になる。

```rust
#[derive(Debug, PartialEq)]
// Debug: フォーマッタを利用して値を出力できるようにする。 PartialEq: 構造体の各要素を比較できるようにする

struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    use super::*; // 外側の要素を利用
    #[test]
    fn item_test(){
        let apple1 = GItem{
            name: String::from("apple"),
            price: 2400,
        };
        let mut apple2 = GItem{
            name: "apple".to_string(),
            price: 0,
        };
        apple2.price = 2400;

        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);

        assert_eq!(apple1, apple2);
    }
}
```



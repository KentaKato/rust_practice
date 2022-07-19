# 所有権システムについて

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
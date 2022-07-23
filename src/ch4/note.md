# 構造体とメソッド

Rustにはクラスがない。

```rust
fn main(){
    let body = Body::new(163.0, 75.2, "田中");
    body.print_result();
    let body = Body::new(158.2, 55.0, "スズキ");
    body.print_result();
    let body = Body::new(174.2, 54.2, "井上");
    body.print_result();
}

struct BmiRange{
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange{
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange{min, max, label: label.to_string()}
    }

    fn test(&self, v: f64) -> bool {
        (self.min <= v) && (v < self.max)
    }
}

struct Body{
    height: f64,
    weight: f64,
    name: String,
}

impl Body {
    fn new(height: f64, weight: f64, name: &str) -> Self {
        Body{height, weight, name: name.to_string()}
    }

    fn calc_bmi(&self) -> f64{
        self.weight / (self.height / 100.0).powf(2.0)
    }

    fn print_result(&self){
        let bmi = self.calc_bmi();

        let bmi_list = [
            BmiRange::new(0.0, 18.5, "低体重"),
            BmiRange::new(18.5, 25.0, "普通体重"),
            BmiRange::new(25.0, 30.0, "肥満1度"),
            BmiRange::new(30.0, 35.0, "肥満2度"),
            BmiRange::new(35.0, 40.0, "肥満3度"),
            BmiRange::new(40.0, 99.0, "肥満4度"),
        ];
        let mut result = String::from("不明");

        for range in bmi_list{
            if range.test(bmi){
                result = range.label.clone();
            }
        }
        println!("{}さん, BMI={}, 判定={}", self.name, bmi, result);
    }
}
```

# 構造体更新記法
構造体のオブジェクトを生成する際、他のオブジェクトのフィールドの値を用いて初期化する記法
`..オブジェクト名`とする。

```rust
struct Person{
    name: String,
    age: i32,
}

impl Person{
    fn new(name: &str, age: i32) -> Self{
        Person{name: name.to_string(), age}
    }
}

fn main() {
    let taro = Person::new("Taro", 18);
    let jiro = Person{
        name: String::from("Jiro"),
        ..taro // 構造体更新記法
    };

    println!("{}, {}", taro.name, taro.age);
    println!("{}, {}", jiro.name, jiro.age);
}
```

# トレイト
構造体に持たせたいメソッドを箇条書したもの。
異なる型に対し、共通のメソッドを定義するために使用。

```rust
trait TreasureBox {
    fn open(&self, key_no:i32) -> bool;
    fn check(&self);
}

struct JewelryBox{
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool{
        self.key_no == key_no
    }
    fn check(&self) {
        println!("宝石箱だった！金貨{}枚入手！", self.price);
    }
}

struct TrapBox{
    damage: i32,
}

impl TreasureBox for TrapBox{
    fn open(&self, _key: i32) -> bool{ // アンダースコアから始まる変数は、未使用でもコンパイル時に警告が出ない
        return true;
    }
    fn check(&self){
        println!("罠だった。{}のダメージ！", self.damage);
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32){ // TreasureBox traitを実装している構造体のみ引数に取れる
    if !tbox.open(key_no){
        println!("鍵が合わず宝箱が開きません。");
        return;
    }
    tbox.check();
}

fn main(){
    let box1 = JewelryBox{price: 30, key_no: 1};
    let box2 = TrapBox{damage: 100000000};
    let box3 = JewelryBox{price: 20, key_no: 2};
    let my_key = 2;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}
```


# ジェネリクス
C++のテンプレートみたいなもの。
C++のテンプレートのように任意の方を許容するのではなく、「型が実装されているべきトレイト」を指定する。

書式
```rust
fn func_name <T: トレイト> (arg1: T, arg2: T, ...) -> 戻り値の型 {
    ...
}
// または
fn func_name <T> (arg1: T, arg2: T, ...) -> 戻り値の型 {
    where T: トレイト
    ...
}
```

例１
```rust
fn add <T: std::ops::Add<output=T>> (a: T, b: T) -> T{
    a + b
}
// または
fn add <T> (a: T, b: T) -> T{
    where: T: std::ops::Add<output=T>
    a + b
}
```


上記のstd::ops::Add<output=T>は
- std::ops
    - 標準ライブラリーのうち、オーバーロード可能な演算子のトレイトを定義したモジュール
- std::ops::Add
    - 足し算を表すトレイト

⇒つまり、足し算を実装した型のみ指定できるということになる。
したがって、add('a', 'b')はエラーになる。


例２
```rust
// 2倍する関数
fn x2 <T: std::ops::Add<Output=T> + Copy> (n: T) -> T{
    n + n
}
```
+演算子を用いることで、複数のトレイトを指定できる。
同じ変数を２度繰り返し使用することになるので、Copyトレイトが実装されている必要がある（？？？）


## トレイト境界
ジェネリック型にトレイトを指定することを「トレイト境界」と呼ぶ。
どんな方でも指定できるはずのジェネリック型にトレイトを指定することで制約を課す。


## 構造体でのジェネリクス指定

```rust
#[derive(Debug)]
struct Point<T>
{
    x: T,
    y: T,
}

fn main(){
    let pt_i = Point{x: 20, y: 50};
    let pt_f = Point{x: 20.0, y: 50.0};
}
```

※構造体の直前に書いた#[derive(Debug)]により、構造体のフィールドの各値をprintln!で出力できるようになる。


```rust
#[derive(Debug)]
struct Point<T>
{
    x: T,
    y: T,
}

impl<T> Point<T> where T: std::ops::AddAssign{
    fn new(x: T, y: T) -> Self{
        Self {x, y}
    }

    fn add(&mut self, pt: Point<T>){ // 構造体のフィールド値を変更する場合は、&mut selfのように指定してミュータブルを明示。
        self.x += pt.x;
        self.y += pt.y;
    }
}
```

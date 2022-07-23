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
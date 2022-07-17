use std::collections::HashMap;
fn main(){
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 30);

    match map.get("D"){
        None => println!("D does not exist."),
        Some(v) => println!("D={}", v),
    }
}
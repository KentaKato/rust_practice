fn main(){
    let mut g1 = String::from("sample");
    g1 = show_message(g1);
    println!("{}", g1);
}

fn show_message(message: String) -> String{
    println!("{}", message);
    return message;
}
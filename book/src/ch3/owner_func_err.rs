fn main(){
    let g1 = String::from("sample");
    show_message(g1);
    println!("{}", g1);
}

fn show_message(message: String){
    println!("{}", message)
}
use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Specify the file name to be read.");
        return;
    }

    let filename = &args[1];
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}
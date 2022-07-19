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
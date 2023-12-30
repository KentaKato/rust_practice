
fn set_100(v: &mut u32)
{
    *v = 100;
}

fn set_200(v: &mut u32)
{
    *v = 200;
}

fn main()
{
    let mut v: u32 = 0;
    println!("{}", v);
    set_100(&mut v);
    println!("{}", v);
    set_200(&mut v);
    println!("{}", v);
}
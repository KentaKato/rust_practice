fn main()
{
    for x in 1..10
    {
        for y in 1..10
        {
            // print!で改行しない
            print!("{:3},", x * y);
        }
        println!("");
    }
}
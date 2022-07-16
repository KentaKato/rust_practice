fn main()
{
    let pc_price = 98000.0;
    let a_ship_price = 1200.0;
    let a_rate = 0.8;
    let b_ship_price = 0.0;
    let b_rate = 0.9;

    println!("Total cost of shop A: {}yen.", pc_price * a_rate + a_ship_price);
    println!("Total cost of shop B: {}yen.", pc_price * b_rate + b_ship_price);
}
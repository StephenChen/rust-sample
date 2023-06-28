const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    let mut xx: u8 = 254;
    xx = xx + 1 + 1;
    println!("The value of x is: {x}");
}

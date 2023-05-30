use my_crate::kinds::PrimaryColor;
use my_crate::mix;

fn main() {
    println!("Hello, world!");

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

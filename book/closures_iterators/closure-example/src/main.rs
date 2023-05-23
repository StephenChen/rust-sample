use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(6);

    let mut list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

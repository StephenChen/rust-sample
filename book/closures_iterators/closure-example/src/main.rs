use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

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
    // thread::spawn(move || println!("From thread: {:?}", list));
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);


    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    borrows_mutably();
    println!("After calling closure: {:?}", list);


    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

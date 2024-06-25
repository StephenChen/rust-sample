use std::thread;

fn main() {
    println!("Hello, world!");

    let list = vec![1, 2, 3];
    println!("before: {list:?}");

    thread::spawn(move || println!("thread: {list:?}")).join().unwrap();

    // println!("after: {list:?}");

    let mut v1: Vec<i32> = vec![1, 2, 3];

    v1.iter_mut().for_each(|x| *x += 1);

    println!("{:?}", v1);
}

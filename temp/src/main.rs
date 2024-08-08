use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
vec![]
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", counter.lock().unwrap());

    // println!("Hello, world!");
    //
    // let list = vec![1, 2, 3];
    // println!("before: {list:?}");
    //
    // thread::spawn(move || println!("thread: {list:?}")).join().unwrap();
    //
    // // println!("after: {list:?}");
    //
    // let mut v1: Vec<i32> = vec![1, 2, 3];
    //
    // v1.iter_mut().for_each(|x| *x += 1);
    //
    // println!("{:?}", v1);
}

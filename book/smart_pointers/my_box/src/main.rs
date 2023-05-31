use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {name}");
}

fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let bx = 5;
    let by = MyBox::new(bx);
    assert_eq!(5, bx);
    assert_eq!(5, *by);

    let m = MyBox::new(String::from("rust"));
    hello(&**m);
    hello(&m);
    hello(&(*m)[..])
}

fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = PointOne { x: 5, y: 10 };
    let float = PointOne { x: 1.0, y: 4.0 };
    let both_integer = PointTwo { x: 5, y: 10 };
    let both_float = PointTwo { x: 1.0, y: 4.0 };
    let integer_and_float = PointTwo { x: 5, y: 4.0 };

    let p = PointOne { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointTwo { x: 5, y: 10.4 };
    let p2 = PointTwo { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct PointOne<T> {
    x: T,
    y: T,
}

impl<T> PointOne<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl PointOne<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointTwo<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> PointTwo<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointTwo<X2, Y2>) -> PointTwo<X1, Y2> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
}



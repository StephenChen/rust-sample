fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let scale = 2;
    let rectangle1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_rectangle(&rectangle1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );
    if rectangle1.width() {
        println!("The rectangle has a nonzero width; it is {}", rectangle1.width);
    }
    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rectangle1.can_hold(&rectangle2));

    dbg!("rectangle1 is {}",&rectangle1);
    println!("rectangle1 is {:#?}", rectangle1);

    let sq = Rectangle::square(3);
    println!("square is {:#?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    println!("Hello, world!");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello Macro")
    }
}


fn main() {
    Pancakes::hello_macro();
}

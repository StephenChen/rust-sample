fn main() {
    let mut s = String::from("asd");
    s.push_str(", qwe");
    takes_ownership(s.clone());
    change(&mut s);

    println!("1: {s}");
    println!("2: {s}");

    let x = 5;
    let y = x;
    println!("x: {x}, y: {y}");

    let s1 = String::from("qwe");
    let s2 = s1.clone();
    println!("s1: {s1}, s2: {s2}");

    let mut ss = String::from("zxc");
    let r1 = &mut ss;
    println!("{}", r1);
    let r2 = &mut ss;
    println!("{}", r2);
}

fn takes_ownership(_s: String) {}

fn change(s: &mut String) {
    s.push_str("asd");
}


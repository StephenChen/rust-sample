fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let mut v5 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v5[100];
    // let does_not_exist = v5.get(100);

    let mut v11 = vec![100, 32, 57];
    for x in &v11 {
        println!("{x}")
    }
    for i in &mut v11 {
        *i += 50;
        println!("{i}")
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("ads")),
    ];
}

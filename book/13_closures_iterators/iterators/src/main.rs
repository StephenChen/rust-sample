fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v1.iter().map(|x| x + 1).collect::<Vec<_>>());
}

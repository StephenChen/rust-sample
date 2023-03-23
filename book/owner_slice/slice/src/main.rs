fn main() {
    let s = String::from("qwe asd zxc");

    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);
    let word = first_word(&s);

    let my_string_literal = "qwe asd zxc";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

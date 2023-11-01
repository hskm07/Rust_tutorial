fn main() {

    let k = String::from("hello world");
    let hello = &k[0..5];
    let world = &k[6..11];
    println!("{}, {}", hello, world);

    let m = String::from("hello");
    let len = m.len();
    let slice = &m[3..len];
    println!("{}", slice);
    let slice = &m[3..];
    println!("{}", slice);
    let slice = &m[..];
    println!("{}", slice);

    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("{}", word);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    let word = first_word(my_string_literal);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

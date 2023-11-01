fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);
    println!("the length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    println!("{}", r1);

    let _reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

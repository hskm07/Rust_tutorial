fn main() {
    let s = concat!("A", "b2", 3);
    let s = format!("{}--{:?}", s, ("D", 5));
    let s = format!("{}-{}", "abc", "def");
    println!("{}", s);
    
    println!("defined in file : {}", file!());
    println!("defined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    // assertion
    // cargo run >> FAILED
    // cargo run --release >> SUCECESS
    assert!(true);
    assert_eq!(1,1);
    assert_ne!(1,0);
    debug_assert!(false);
    debug_assert_eq!(1,1);
    debug_assert_ne!(1,0);
}

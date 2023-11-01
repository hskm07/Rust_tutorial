use std::io;

fn main() {
    let sum = 5 + 10;
    println!("{}", sum);
    let difference = 95.5 - 30.5;
    println!("{}", difference);
    let product = 4 * 30;
    println!("{}", product);
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
    let remainder = 43 % 5;
    println!("{}", remainder);

    let t = true;
    let f: bool = false;
    println!("{}, {}", t, f);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    println!("{}", five_hundred);
    let six_point_four = x.1;
    println!("{}", six_point_four);
    let one = x.2;
    println!("{}", one);

    let a = [1,2,3,4,5];
    println!("{:?}", a);
    let b = [3; 5];
    println!("{:?}", b);

    //配列要素への無効なアクセス
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}

fn main() {
    // macth
    let i: i32 = 1;
    match i { 
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"), // underscores match any value.
    }

    // mtach + enum
    enum Color {
        Red,
        Blue,
        Green,
    }

    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }

    let result: Result<i32, String> = Ok(100);
    let result_number = match result{
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        }
    };
    println!("{}", result_number);

    // Range
    for number in 1..5{
        println!("{}", number);
    }

    // Iterator
    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }
}

struct Iter{
    current: usize,
    max: usize,
}

impl Iterator for Iter{
    type Item = usize;

    // next()
    fn next(&mut self) -> Option<usize>{
        self.current += 1;
        if self.current -1 < self.max {
            Some(self.current -1)
        }else{
            None
        }
    }
}
fn main() {
    let mut count = 0;

    /* loop */
    let result = loop{
        println!("count: {}", count);
        count += 1;
        if count == 10{
            break count;
        }
    };

    /* while */
    let mut count_w = 0;
    while count_w < 10 {
        println!("count_w: {}", count_w);
        count_w += 1;
    }

    /* for */
    let count_f: i32;

    for count_f in 0..10 {
        println!("count: {}", count_f);
    }

    let array=[0,1,2,3,4,5,6,7,8,9];
    for element in &array {
        println!("element: {}", element);
    }

    /* loop + label */
    'main: loop{
        println!("main loop start:");
        'sub: loop {
            println!("sub loop start");

            break 'main;
            println!("sub loop end"); // Don't show result
        }
        println!("main loop end"); // Don't show result
    }

    
}

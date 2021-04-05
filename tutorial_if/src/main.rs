fn main() {
    /* exapmle 1 */
    // let number = 1;
    // if 0 < number {
    //     println!("0 < number");
    // }else if number < 0{
    //    println!("number < 0"); 
    // }else {
    //     println!("0 == number");
    // }

    /* exapmle2 */
    let number = 1;
    let result = if 0 <= number {
        number
    }else{
        -number
    };
    println!("{0}", result);

}

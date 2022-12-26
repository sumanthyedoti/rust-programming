const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("const THREE_HOURS_IN_SECONDS = {THREE_HOURS_IN_SECONDS}");
    let x = 5;
    println!("The value of x is: {}", x);
    /* x = 6;
      Error: immutable
    */

    /* mutable */
    // can mutate with a value of same type
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 10;
    println!("y again: {}", y);

    /* shadowing */
    let spaces = "   ";
    print_type_of(&spaces); // string
    let spaces = spaces.len(); // number
    println!("spaces again: {}", spaces);
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    /* overflow error */
    // let o: u8 = 267;
}

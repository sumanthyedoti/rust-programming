fn return_five() -> i32 {
    5
}

fn _do_not_return_five() -> () {
    // If you add a semicolon to the end of an expression,
    // you turn it into a statement, which will then not return a value
    5;
}

fn six() -> i32 {
    // use return only for explicit early return
    return 6;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    print_int(6);
    print_labeled_measurement(5, 'm');

    let x = 5;
    println!("x = {x}");
    // same as
    let x = { 5 };
    println!("x = {x}");

    // fucntions call is an expression
    let _x: () = print_int(3);
    /* blocks */
    // blocks are also expressions
    // The block that we use to create new scopes, {}, is an expression
    let _x: () = {};
    let _y = {
        let x = 3;
        // expression, return a value
        x + 2 // return value
    }; // 5

    println!("five is {}", return_five());
    println!("six is {}", six());
    println!("6 + 1 = {}", plus_one(6));
}

fn print_int(x: u32) -> () {
    println!("x = {x}");
}

fn print_labeled_measurement(value: i32, label: char) {
    println!("{value}{label}");
}

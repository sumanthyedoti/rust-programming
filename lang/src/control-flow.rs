fn main() {
    /* if */
    let x = 6;
    if (x > 5) && (2 < 3) {
        println!("{}", x);
    }
    // if is an experssion
    // each arm of the if must be the same type
    let number = if 5 > 6 { 5 } else { 6 };
    println!("{number}");

    /* loop */
    println!("");
    let mut loop_var = 0;
    loop {
        loop_var += 1;
        println!("loop-1.{loop_var}");
        if loop_var == 4 {
            break;
        }
    }

    // return value in loop
    println!("");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop2 result: {}", result);

    /* nested loops and loop labels */
    // break and continue apply to the innermost loop
    println!("");
    let mut x = 0;
    loop {
        x += 1;
        if x > 3 {
            break;
        };
        let mut y = 0;
        loop {
            y += 1;
            if y > 3 {
                break;
            };
            println!("x-y -- {x}-{y}");
        }
    }
    //can optionally specify a loop label on a loop that you can then use with break or continue
    // to specify that those keywords apply to the labeled loop instead of the innermost loop
    println!("");
    let mut x = 0;
    'outer: loop {
        x += 1;
        if x > 3 {
            break;
        };
        let mut y = 0;
        loop {
            y += 1;
            if y > 3 {
                break 'outer;
            };
            println!("x-y = {x}-{y}");
        }
    }

    /* while */
    println!("");
    counter = 0;
    while counter != 10 {
        counter += 1;
    }
    println!("after while loop: {}", counter);

    /* for */
    // for is less error-prone and more clean than while
    println!("");
    let ar = [10, 20, 30, 40, 50];
    for element in ar.iter() {
        println!("the value is: {}", element);
    }

    /* for with range */
    println!("");
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("\nReversed range");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    // step by
    println!("");
    println!("\nStep By");
    for x in (1..10).step_by(3) {
        println!("{}", x);
    }
    // skip
    println!("");
    println!("\nSkip");
    for x in (1..10).skip(6) {
        println!("{}", x);
    }
}

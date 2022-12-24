fn annotate(str: String) {
    print!("\n");
    println!("/* {} */", str);
    let underline_length = str.chars().count() + 6;
    println!("{}", "-".repeat(underline_length));
}

fn main() {
    format!("Hello World\nhii");

    // formate
    annotate(String::from("formate"));
    println!("{} days", 31);

    // the variable name can go inside the curly brackets
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);

    //Positional arguments
    annotate(String::from("Positional arguments"));
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //named arguments
    annotate(String::from("named arguments"));
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    //Special formatting can be specified after a `:`.
    //:b -> binary
    annotate(String::from("Special formatting, :b -> binary"));
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // padding
    annotate(String::from("padding"));
    print!("{number:>width$}", number = 1, width = 6);

    // fill padding
    annotate(String::from("fill padding"));
    println!("{number:>0width$}", number = 1, width = 3);

    //precision
    annotate(String::from("precision"));
    println!("PI is roughly {:.1$}", 3.141592, 3);
    println!("PI is roughly {number:.prec$}", number = 3.141592, prec = 3);
}

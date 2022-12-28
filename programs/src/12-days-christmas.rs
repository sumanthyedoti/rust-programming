use std::env;

fn main() {
    let num_of_days = match env::args().nth(1) {
        Some(n) => n,
        None => String::from("12"),
    };
    let num_of_days: usize = match num_of_days.trim().parse() {
        Ok(n) if n > 12 => 12,
        Ok(n) => n,
        Err(_) => 12,
    };

    for i in 0..num_of_days {
        println!("");
        println!(
            "On the {} day of Christmas, my true love sent to me",
            nth_word(i)
        );
        for j in (1..=i).rev() {
            println!("{}", gifts(j + 1));
        }
        if i == 0 {
            println!("A {}", gifts(1));
        } else {
            println!("And a {}", gifts(1));
        }
    }
}

fn gifts(day: usize) -> String {
    match day {
        1 => String::from("partridge in a pear tree"),
        2 => String::from("Two turtledoves"),
        3 => String::from("Three French hens"),
        4 => String::from("Four calling birds"),
        5 => String::from("Five gold rings"),
        6 => String::from("Six geese a-laying"),
        7 => String::from("Seven swans a-swimming"),
        8 => String::from("Eigth maids a-milking"),
        9 => String::from("Nine ladies dancing"),
        10 => String::from("Ten lords a-leaping"),
        11 => String::from("Eleven pipers piping"),
        12 => String::from("Twelve drummers drumming"),
        _ => String::from(""),
    }
}
fn nth_word(i: usize) -> String {
    match i {
        1 => String::from("first"),
        2 => String::from("second"),
        3 => String::from("third"),
        4 => String::from("fourth"),
        5 => String::from("fifth"),
        6 => String::from("sixth"),
        7 => String::from("seventh"),
        8 => String::from("eigtht"),
        9 => String::from("ninth"),
        10 => String::from("tenth"),
        11 => String::from("eleventh"),
        12 => String::from("twelfth"),
        _ => String::from(""),
    }
}

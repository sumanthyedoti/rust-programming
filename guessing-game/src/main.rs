use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn flush() {
    io::stdout().flush().unwrap();
}

fn main() {
    let computer_guess = rand::thread_rng().gen_range(1..=100); // inclusive on the lower and upper bounds
    println!("Guess the number!");

    print!("Guess it: ");
    flush();

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing the variable name, a new variable
        // 'parse()' need to know the data-type to parse so, to annotate the type of 'guess'
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🙅 {}", "That's not a number for me!".red());
                print!("Try again: ");
                flush();
                continue;
            }
        };

        match guess.cmp(&computer_guess) {
            Ordering::Less => {
                print!("{}", "bigger: ".magenta());
                flush();
            }
            Ordering::Greater => {
                print!("{}", "smaller: ".magenta());
                flush();
            }
            Ordering::Equal => {
                println!("{}", "correct!🤠".green().bold());
                break;
            }
        }
    }
}

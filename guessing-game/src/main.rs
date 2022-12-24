use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let computer_guess = rand::thread_rng().gen_range(1..=100); // inclusive on the lower and upper bounds

    println!("Guess the number: ");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("That's not a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number for me! Try again..");
                continue;
            }
        };

        match guess.cmp(&computer_guess) {
            Ordering::Less => println!("bigger.."),
            Ordering::Greater => println!("smaller.."),
            Ordering::Equal => {
                println!("correct!");
                break;
            }
        }
    }
}

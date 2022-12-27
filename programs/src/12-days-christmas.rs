use std::env;

fn main() {
    struct Day {
        num_word: String,
        nth_day: String,
        thing: String,
    }
    let num_of_days = match env::args().nth(1) {
        Some(n) => n,
        None => String::from("12"),
    };
    let num_of_days: usize = match num_of_days.trim().parse() {
        Ok(n) if n > 12 => 12,
        Ok(n) => n,
        Err(_) => 12,
    };
    let days: [Day; 12] = [
        Day {
            num_word: String::from("One"),
            nth_day: String::from("first"),
            thing: String::from("patridge in a pear tree"),
        },
        Day {
            num_word: String::from("Two"),
            nth_day: String::from("second"),
            thing: String::from("turtledoves"),
        },
        Day {
            num_word: String::from("Three"),
            nth_day: String::from("third"),
            thing: String::from("French hens"),
        },
        Day {
            num_word: String::from("Four"),
            nth_day: String::from("fourth"),
            thing: String::from("calling birds"),
        },
        Day {
            num_word: String::from("Five"),
            nth_day: String::from("fifth"),
            thing: String::from("gold rings"),
        },
        Day {
            num_word: String::from("Six"),
            nth_day: String::from("sixth"),
            thing: String::from("geese a-laying"),
        },
        Day {
            num_word: String::from("Seven"),
            nth_day: String::from("seventh"),
            thing: String::from("swans a-swimming"),
        },
        Day {
            num_word: String::from("Eigth"),
            nth_day: String::from("eighth"),
            thing: String::from("maids a-milking"),
        },
        Day {
            num_word: String::from("Nine"),
            nth_day: String::from("ninth"),
            thing: String::from("ladies dancing"),
        },
        Day {
            num_word: String::from("Ten"),
            nth_day: String::from("tenth"),
            thing: String::from("lords a-leaping"),
        },
        Day {
            num_word: String::from("Eleven"),
            nth_day: String::from("eleventh"),
            thing: String::from("pipers piping"),
        },
        Day {
            num_word: String::from("Twelve"),
            nth_day: String::from("Twelfth"),
            thing: String::from("drummers drumming"),
        },
    ];

    for i in 0..num_of_days {
        println!("");
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[i].nth_day
        );
        for j in (1..=i).rev() {
            println!("{} {}", days[j].num_word, days[j].thing);
        }
        if i == 1 {
            println!("A partridge in a pear tree");
        } else {
            println!("And a partridge in a pear tree");
        }
    }
}

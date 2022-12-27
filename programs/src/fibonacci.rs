use std::env;

fn main() {
    let arg = match env::args().nth(1) {
        Some(n) => n,
        None => String::new(),
    };
    let argument: u8 = match arg.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid argument");
            return;
        }
    };
    const MAX: u8 = 93;
    if argument > MAX {
        println!("Please do not exeed {MAX}");
        return;
    }
    println!("{}", fibnoacci_tail(argument));
}

// recursive
fn _fibnoacci_rec(n: u8) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => _fibnoacci_rec(n - 2) + _fibnoacci_rec(n - 1),
    }
}

// tail recursion
fn fibnoacci_tail(num: u8) -> u64 {
    if num == 0 {
        return 0;
    };
    fn rec(i: u8, num: u8, n1: u64, n2: u64) -> u64 {
        if i == num {
            return n2;
        };
        return rec(i + 1, num, n2, n1 + n2);
    }
    return rec(1, num, 0, 1);
}

// mutating loop
fn _fibnoacci_mut(num: u8) -> u64 {
    let mut i = 1;
    let mut n1: u64 = 0;
    let mut n2: u64 = 1;
    while i < num {
        let sum = n1 + n2;
        n1 = n2;
        n2 = sum;
        i += 1;
    }
    return n2;
}

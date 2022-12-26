fn main() {
    /* INTEGERS */
    // [u/i]8, 16, 32, 64
    // arch dependent (32/64), [u/i]size
    // i32 is default
    /* FLOATS */
    // f32, f64. f64 is default
    // All floating-point types are signed
    let i = 43
    let f = 65.56
    println!("{i}, {f}")
    /*char */
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("chars: {}, {}, {}", c, z, heart_eyed_cat);

    /*tuples */
    let a_tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t_x, t_y, t_z) = a_tup;
    println!("tuple values, {} {} {}", t_x, t_y, t_z);
    println!("tuple using . - {} {} {}", a_tup.0, a_tup.1, a_tup.2);

    /*arrays */
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _a1: [i32; 5] = [1, 2, 3, 4, 5];
    let _a2 = [3; 5]; // [3, 3, 3, 3, 3]
    println!("tuple values, {}", months[11]);
}

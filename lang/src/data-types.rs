fn main() {
    /* INTEGERS */
    // [u/i]8, 16, 32, 64
    // arch dependent (32/64), [u/i]size
    // i32 is default
    /* FLOATS */
    // f32, f64. f64 is default
    // All floating-point types are signed
    let i = 43;
    let f: f64 = 5.0 / 3.0;
    println!("{i}, {f}");
    /*char */
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("chars: {}, {}, {}", c, z, heart_eyed_cat);

    /* boolean */
    let _t: bool = true;
    let _f = false;

    /*tuples */
    let a_tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t_x, t_y, t_z) = a_tup;
    println!("destructured tuple values, {} {} {}", t_x, t_y, t_z);
    println!("tuple values using . - {} {} {}", a_tup.0, a_tup.1, a_tup.2);
    // unit
    // a tuple without any values is 'unit'
    // This value and its corresponding type are both written () and represent an empty value or an empty return type
    let _unit: () = ();

    /*arrays */
    // Arrays in Rust have a fixed length
    // Arrays are useful when you want your data allocated on the stack rather than the heap
    // or when you want to ensure you always have a fixed number of elements.
    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
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

    /* string */
    let str: &str = "String";
    println!("{str}");
}

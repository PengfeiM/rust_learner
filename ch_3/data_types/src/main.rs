use std::any::type_name_of_val;

fn main() {
    // Rust is a statically typed language, the type of vars must be known when compiling.
    // compiler can figure out the type of vars according to the value set to it.
    // but when many types are possible, we must annotate type in code.
    // let guess = "42".parse().expect("Not a number!");
    /*
    cargo build
       Compiling data_types v0.1.0 (/home/revan_m/codes/rust_begginer/rust_learner/ch_3/data_types)
    error[E0284]: type annotations needed
     --> src/main.rs:2:9
      |
    2 |     let guess = "42".parse().expect("Not a number!");
      |         ^^^^^        ----- type must be known at this point
      |
      = note: cannot satisfy `<_ as FromStr>::Err == _`
    help: consider giving `guess` an explicit type
      |
    2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
      |              ++++++++++++

    For more information about this error, try `rustc --explain E0284`.
    error: could not compile `data_types` (bin "data_types") due to 1 previous error
    */
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {guess}");
    // ===========================================================================================
    // Scalar Types
    // scalar type represents a single value.
    // Four primary scalar types in Rust:
    // 1. int
    // 2. float
    // 3. bool
    // 4. char

    // 1. int
    let i: i32 = 12345;
    let type_i = type_name_of_val(&i);
    println!("i: {i}, type: {type_i}");

    // 2. float
    let ff = 2.0; // f64
    let type_ff = type_name_of_val(&ff);
    println!("ff: {ff}, type of ff: {type_ff}");
    let f: f32 = 3.0; //f32
    let type_f = type_name_of_val(&f);
    println!("f: {f}, type of f: {type_f}");

    // 3. bool
    let t = true;
    let type_t = type_name_of_val(&t);
    let f = false;
    let type_f = type_name_of_val(&f);
    println!("t: {t}, type of t: {type_t}");
    println!("f: {f}, type of f: {type_f}");

    // 4. char
    let c = 'z';
    let type_c = type_name_of_val(&c);
    let z: char = 'ℤ'; // with explicit type annotation
    let type_z = type_name_of_val(&z);
    let heart_eyed_cat = '😻';
    let type_emoji = type_name_of_val(&heart_eyed_cat);
    println!("c: {t}, type of c: {type_c}");
    println!("z: {z}, type of z: {type_z}");
    println!("emoji: {heart_eyed_cat}, type of f: {type_emoji}");
    // ===========================================================================================

    //  Rust has two primitive compound types: tuples and arrays.

    // 1. Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1); // no type is fine.
    let (x, y, z) = tup;
    println!("tuple (x,y,z): ({x}, {y}, {z})");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    // access an element of tuple by operator(.)
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("access tuple: ({five_hundred}, {six_point_four}, {one})");

    // 2. Array
    // size unchangable, for more flexibility, vector will come later.
    let a = [1, 2, 3, 4, 5];
    println!("An array: {:#?}", a);

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
    println!("Months: {:#?}", months);

    // declare an array with same value
    let aa = [3; 5];
    println!("An array with same elements: {:#?}", aa);

    // access elements of array
    println!("The 2nd elem of array {:?}: {}", a, a[1]);
}

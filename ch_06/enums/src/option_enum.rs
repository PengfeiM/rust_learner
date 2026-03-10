/*
* Option is a special kind of enum defined by the standard library.
* Null is a dangerous thing in programming, it can cause severe run-time error.
* As such, Rust doesn't have nulls, but Rust has a special kind of enum, Option, that can express
* the absence of a value.
* enum Option<T>{
*   None,
*   Some(T),
* }
* */

pub fn some_example() {
    println!("some basic **Option**");
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("Option<i32>: {:?}", some_number);
    println!("Option<char>: {:?}", some_char);
    println!("Option<i32> -> None: {:?}", absent_number);

    let x = 8;
    // take value out of Some() before use it.
    let y = some_number.expect("get a number");
    println!("i32 + Some(i32).expect= {}", x + y);
}

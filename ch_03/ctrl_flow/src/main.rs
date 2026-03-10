fn main() {
    // branch
    if_statement();

    // loop
    loop_statement();
    while_statement();
    for_statement();
}

fn if_statement() {
    println!("Input a number");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");
    let number: i32 = input.trim().parse().expect("Invalid number");
    println!("number is {number}, compare to 5");
    if number < 5 {
        println!("number is smaller than 5");
    } else if number == 5 {
        println!("number equals to 5");
    } else {
        println!("number is bigger than 5");
    }

    /*
    if number{}
    */
    // condition must be a bool statement, if you pass another type, you will see something like
    // this.
    /*
        Compiling ctrl_flow v0.1.0 (/home/revan_m/codes/rust_begginer/rust_learner/ch_3/ctrl_flow)
    error[E0308]: mismatched types
    --> src/main.rs:17:8
    |
    17 |     if number{}
    |        ^^^^^^ expected `bool`, found integer

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `ctrl_flow` (bin "ctrl_flow") due to 1 previous error
    */

    // use if in let statement.
    let condition = true;
    let number = if condition { 5 } else { 6 }; // the type must be the same;
    println!("The value of number is: {number}");
}

fn loop_statement() {
    /*
     * loop{println!("dddd");}
     */

    //return value with loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // normally, you can only jump out one layer of loop by break.
    // but with loop label, you can jump out to anywhere you like.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_statement() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
}

fn for_statement() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // too complex
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // easy way
    for element in a {
        println!("the value is: {element}");
    }

    // in a array and reverse it
    for number in (1..4).rev() {
        println!("{number}");
    }
}

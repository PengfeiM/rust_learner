fn main() {
    println!("Hello, world!");

    another_function();

    fn_with_param(6);

    print_labeled_measurement(5, 'm');

    // a statement, make y = x + 1,the semicolon must be at the very end.
    let y = {
        let x = 3;
        x + 1 // no semicolon here
    };
    println!("The value of y is: {y}");

    let x = fn_with_return_value();
    println!("The value of x is: {x}");

    let xx = plus_one(x);
    println!("x + 1 = {xx}");
}

// use fn to declare a new function
// Rust use snake_case for name of function and vars.
fn another_function() {
    println!("Another function.");
}

// parameters
// fn func_name(param_name: param_type)
fn fn_with_param(x: i32) {
    println!("The value of x is :{x}");
}

// multiple params
// fn func_name(param1_name: param1_type, param2_name, param2_type)
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

// let's make a function with return value
// fn func_name() -> return_type {}
fn fn_with_return_value() -> i32 {
    // return is not needed when return at last.
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // normal variables
    let x = 5;
    println!("The value of x is :{x}");
    // x = 6; // vars without mut immutable, this line will cause an error in compilation.
    // println!("The value of x is :{x}");

    // mutable variables
    let mut y = 5;
    println!("The value of y is :{y}");
    // you can change value of a mut variable.
    y = 6;
    println!("The value of y is :{y}");

    // Constant
    // 1. constant is IMMUTABLE and can't be set to MUT.
    // 2. the TYPE of constant must be annotated.
    // 3. constant can be declared in ANY scope, useful when we need global vars.
    // 4. constant can only be set to a CONSTANT value, not some result of expression in run-time.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constant THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // shadowing
    // to change a var's value
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is :{x}");
    // diff with mut
    //      1. only change value in let, mut can be changed anywhere.
    //      2. shadowing creates a new var with the same name, so we can change the TYPE of the var.
    let spaces = "      ";
    let spaces = spaces.len();
    println!("The number of spaces is {spaces}");
    // let mut spaces = "      ";
    // spaces = spaces.len(); // you can't change a mut's type by assign.
    /*
    cargo build
       Compiling vars_mut v0.1.0 (/home/revan_m/codes/rust_begginer/rust_learner/ch_3/vars_mut)
    error[E0308]: mismatched types
      --> src/main.rs:37:14
       |
    36 |     let mut spaces = "      ";
       |                      -------- expected due to this value
    37 |     spaces = spaces.len();
       |              ^^^^^^^^^^^^ expected `&str`, found `usize`

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `vars_mut` (bin "vars_mut") due to 1 previous error
    */
}

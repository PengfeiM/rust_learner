// to do some io with terminal like let user to input a string, we need input/ouput library from standard library.
use std::io;

// let's make a guessing game.
fn main() {
    println!("Guess the number!");

    println!("Please input your guessses.");

    // let is a keyword for create a variable.
    // mtu makes the variable mutable, which means its value can be changed.
    // without it, the variable will be immutable.
    let mut guess = String::new();

    // use stdin in io library
    // an alternative way is 'std::io::stdin()' without use std::io.
    io::stdin()
        .read_line(&mut guess) // a standard func to read user's input, write into var guess.
        .expect("Failed to read line"); // Fault handling func.

    println!("You guessed: {guess}");
}

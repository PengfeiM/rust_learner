// add comparison

use std::cmp::Ordering;
use std::io;

use rand::Rng;

// let's make a guessing game.
fn main() {
    println!("Guess the number!");

    // immutable
    // thread_rng: return a random number generator.
    // gen_range: generate random number in range.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

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

    // use Shadowing to reset a var.
    let guess: u32 = guess.trim().parse().expect("Please input a number!");

    println!("You guessed: {guess}");

    // do some comparison
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

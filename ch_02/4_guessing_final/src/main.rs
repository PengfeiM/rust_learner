// finish it once for all

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

    // println!("The secret number is: {secret_number}");

    // a dead loop, loop create a infinite loop
    loop {
        println!("Please input your guessses.");

        let mut guess = String::new();

        // use stdin in io library
        // an alternative way is 'std::io::stdin()' without use std::io.
        io::stdin()
            .read_line(&mut guess) // a standard func to read user's input, write into var guess.
            .expect("Failed to read line"); // Fault handling func.

        // let guess: u32 = guess.trim().parse().expect("Please input a number!");
        // we abandon the int parser above, and find a way to deal with invalid input.
        // switch from an expect to a match to handle the error.
        let guess: u32 = match guess.trim().parse() {
            // parse() returns Ok or Err, which are both enum, match is a good way to handle.

            /*
            If parse is able to successfully turn the string into a number, it will return an Ok value that contains
            the resultant number.
            That Ok value will match the first arm’s pattern, and the match expression will just
            return the num value that parse produced and put inside the Ok value.
            That number will end up right where we want it in the new guess variable we’re creating.
            * */
            Ok(num) => num,
            /*
             *
             * */
            Err(e) => {
                println!("Err: {e}");
                println!("Invalid input! please input a number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // do some comparison
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // use break to jump out of loop
            }
        }
    }
}

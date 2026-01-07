#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("value is None!");
            None
        }
        Some(i) => {
            println!("not None, plus one for you");
            Some(i + 1)
        }
    }
}

fn roll1(x: i32) {
    match x {
        1 => println!("Big Failure!"),
        20 => println!("Big Success!"),
        others => println!("you roll a {others}"),
    }
}

fn roll2(x: i32) {
    match x {
        1 => println!("Big Failure!"),
        20 => println!("Big Success!"),
        _ => println!("Reroll!!!"),
    }
}

fn main() {
    println!("Let's try enum with match!");
    let p = Coin::Penny;
    value_in_cents(p);
    let q = Coin::Quarter(UsState::Alaska);
    value_in_cents(q);

    // with match and Option<T>, we can also write some code to deal with None.
    println!("Let's deal with None with match!");
    let five = Some(5);
    println!("six = {}", plus_one(five).expect("5"));
    println!("None = {:?}", plus_one(None));

    // of course, match has a logic to deal with default case.
    // * other;
    // * _ if you don't care the value;
    roll1(1);
    roll2(20);
    roll1(0);
    roll2(0);
}

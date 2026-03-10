#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn exist_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter_1(coin: &Coin) -> Option<String> {
    // this is regular match syntax sugar.
    if let Coin::Quarter(state) = coin {
        if state.exist_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relateively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_2(coin: &Coin) -> Option<String> {
    // this is an evil way, dont't try it!
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.exist_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        None
    }
}

fn describe_state_quarter_3(coin: &Coin) -> Option<String> {
    // a mordern way to implement the some logic in describe_state_quarter_2.
    // let-else statement.
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.exist_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        None
    }
}

fn main() {
    // it's not convenient if we just want deal with Some() with match.
    // let config_max = Some(3u8)
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}");
    //     _ => ();
    // }
    // you see, we have to add `_ => ();` to deal with something we don't care.

    // luckily, we have `if let` statement in Rust.
    // a syntax sugar for match
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // of course, we can add `else` right after if let like if-else.
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        println!("Mismatched!");
        count += 1;
    }
    println!("Mismatched coin count: {count}");

    // let's try three ways to demonstrate  the superiority of let-else statement.
    let coin = Coin::Quarter(UsState::Alabama);
    println!("coin quarter: {:?}", describe_state_quarter_1(&coin));
    println!("coin quarter: {:?}", describe_state_quarter_2(&coin));
    println!("coin quarter: {:?}", describe_state_quarter_3(&coin));
}

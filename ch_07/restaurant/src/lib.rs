mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
mod back_of_house {
    // a pub struct won't make its fields public.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // on the contrary, a pub enum will make all of its variants public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// we can use `use` to take something from a crate.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // you may encounter the error:
    // field `seasonal_fruit` of `Breakfast` is private 37:5:1 rust-analyzer E0616
    // meal.seasonal_fruit = String::from("blueberries");

    // let's see what happens when we use pub enum.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // after `use` something, we can use it direcly with out path.
    hosting::add_to_waitlist();
}

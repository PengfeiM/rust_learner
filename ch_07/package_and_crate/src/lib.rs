mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

/*
* use std::cmp::Ordering;
* use std::io;
* */
// consider they are from  the same crate, we can use them in one-line.
use std::{cmp::Ordering, io};
// what if some like
// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// if we want to import all the public items in a crate.
use std::collections::*;

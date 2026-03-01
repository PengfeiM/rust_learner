// Traits is something like interface, but with some Rust unique features.

use std::fmt::{Debug, Display};
use std::iter::Sum;

// Define a Trait.
/// a Summary trait that provides summarize function.
pub trait Summary {
    /// summarize function.
    fn summarize(&self) -> String;

    /// define a function with default implementation.
    fn default_sum(&self) -> String {
        String::from("(Read more...)")
    }
}

// Now, implement the trait.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    /// rewrite default_sum
    fn default_sum(&self) -> String {
        format!("Override the default sum function, {}", self.summarize())
    }
}

/// we can use the trait as function parameter.
pub fn notify_1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// or we can write it in normal generic format.
/// as the form in notify_1 is a syntax sugar.
pub fn nofity_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// what's the difference between notify_1 and nofity_2?
// let's see.
pub fn notify_1_1(item1: &impl Summary, item2: &impl Display) {}
// as you can see, the sugar form is convenient to use different types.
pub fn notify_2_1<T: Summary>(item1: &T, item2: &T) {}
// we can also define params with multi-traits.
pub fn notify_3(item: &(impl Summary + Display)) {}
pub fn notify_4<T: Summary + Display>(item: &T) {}
pub fn function_1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}
// it's hard to read function_1, use `where` can simplify it.
pub fn function_2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

/// we can return A trait, but you can only return one type that impl the trait in logic.
pub fn return_summary() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The larger number is x = {}", self.x);
        } else {
            println!("The larger number is y = {}", self.y);
        }
    }
}

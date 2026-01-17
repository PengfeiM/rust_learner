mod front_of_house;
use front_of_house::hosting;

fn main() {
    println!("Hello, world!");
    hosting::add_to_waitlist();
}

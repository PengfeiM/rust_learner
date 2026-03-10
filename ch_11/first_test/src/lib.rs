pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
    // format!("Hello")
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess { value }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height // this one can pass
        // self.width < other.width && self.height > other.height // this one won't pass
    }
}

#[cfg(test)]
mod tests {

    use std::result;

    use super::*;

    /// this is a auto-generated test func.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /// let's try a self-define test func.
    #[test]
    fn exploration() {
        let result = add(3, 3);
        assert_eq!(result, 6);
        assert_ne!(result, 4);
        // assert_ne!(result, 6);
    }

    /*
    #[test]
    fn another(){
        panic!("Make this test fail");
    }*/

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was: `{result}`"
        );
    }

    /// try test with should_panic!
    /// we can also use expected to chose panic msg.
    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn greater_than_100() {
        Guess::new(101);
    }

    /// use Result<T, E> to return error instead of panicking.
    #[test]
    fn it_works_too() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
}

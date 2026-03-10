//! now, we will do some method tricks.
//! like function, method declare with `fn`.
//! unlike function, method must be defined within the context of a struct(enum or trait object
//! should also do the trick) and their first parameter is always `self`, which refer to the
//! instance of the struct.

/// Rectangle
/// like what we've learned in last section, define a struct named Rectangle.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// now let make up some method for it.
// to define methods for struct, we start with a keyword `impl`, followed by the name of struct
// `Rectangle`.
impl Rectangle {
    /// area
    /// a method of Rectangle to calculate its area.
    ///
    /// with `&self`, we just do borrowing, not taking the ownership.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// reset
    /// reset Rectangle to 0 * 0.
    ///
    /// with &mut self, we can do some modifications on struct, like reset it to 0.
    fn reset(&mut self) {
        self.width = 0;
        self.height = 0;
    }

    /// width
    /// different from variable/function, field and method can have the same name.
    fn width(&self) -> bool {
        self.width > 0
    }

    /// can_hold
    /// can self hold the other rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // all functions defined in impl `struct` can be called associated functions.
    // in the other hand, a associated function without &self as parameter is not a method.
    // it's often used to create a instance of struct. see a Example below:
    /// square
    /// create a special rectangle: square.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // first try of method.
    let rect1 = Rectangle {
        width: 37,
        height: 73,
    };
    println!(
        "The area of the Rectangle {:?} is {} (calculated by Rectangle{{}}.area())",
        rect1,
        rect1.area() // should be familiar if you learned python, golang, etc.
    );

    // try a getter method.
    let mut rect2 = Rectangle {
        width: 114,
        height: 514,
    };
    println!(
        "The area of the Rectangle {:?} is {} (calculated by Rectangle{{}}.area())",
        rect2,
        rect2.area()
    );
    if rect2.width() {
        println!(
            "The rectangle {:?} has a nonzero width; it's {}",
            rect2, rect2.width
        );
    } else {
        println!("The rectangle {:?} has a zero width!", rect2);
    }
    // try &mut self to do some change on struct.
    rect2.reset();
    println!(
        "The area of the Rectangle {:?} after reset is {} (calculated by Rectangle{{}}.area())",
        rect2,
        rect2.area()
    );
    if rect2.width() {
        println!(
            "The rectangle {:?} has a nonzero width; it's {}",
            rect2, rect2.width
        );
    } else {
        println!("The rectangle {:?} has a zero width!", rect2);
    }

    // try some method with more parameters.
    rect2.width = 10;
    rect2.height = 40;
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "can rect1 {:?} hold rect2 {:?}: {}",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    );
    println!(
        "can rect1 {:?} hold rect3 {:?}: {}",
        rect1,
        rect3,
        rect1.can_hold(&rect3)
    );

    let squa = Rectangle::square(5);
    println!("create a square with Rectangle: {:?}", squa);
}

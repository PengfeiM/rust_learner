fn main() {
    println!("Hello, world!");
    // in the old ways, we implement different functions to get largest elem from list with different
    // types.
    println!(
        "===================================================================================="
    );
    println!("let's try old ways first: largest_i32 and largest_char");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number in {number_list:?} is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char in {char_list:?} is {result}");
    println!(
        "===================================================================================="
    );
    println!(
        "===================================================================================="
    );
    println!("now, let's try generic types");

    let result = largest(&number_list);
    println!("The largest number in {number_list:?} is {result}");
    let result = largest(&char_list);
    println!("The largest char in {char_list:?} is {result}");

    println!(
        "===================================================================================="
    );
    try_generic_method();
    println!(
        "===================================================================================="
    );
    println!("try_mixup_things");
    try_mixup_things();
    println!(
        "===================================================================================="
    );
}

// old ways
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// in the old ways, we implement different functions to get largest elem from list with different
// types.
// as you can see, we've write so many redundant code for the same thing.
// with generic types, it could be much simpler.
// but, with out trait std::cmp::PartialOrd, we will meet the error:
// binary operation `>` cannot be applied to type `&T` 49:17:1 rustc E0369
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// we can try generic types in struct, like below.
struct Point<T> {
    x: T,
    y: T,
}

struct PoinT<T, U> {
    x: T,
    y: U,
}

fn try_generic_struct() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
    // mismatched types 79:38:1 rustc E0308
    // expected integer, found floating-point number
    let will_work = PoinT { x: 5, y: 4.0 };
}

// also, enum will do it, as we've seen before.
/*
* enum Option<T> {
*     Some(T),
*     None,
* }
* enum Result<T, E> {
*     Ok(T),
*     Err(E),
* }
* */

// now, let's try some method.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// we can also specify the type we want to use when implement some method.
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        // pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn try_generic_method() {
    let p = Point { x: 5, y: 10 };

    println!("p.x={}, p.y={}", p.x(), p.y());

    let p_f64 = Point { x: 3.5, y: 4.5 };
    println!(
        "the distance from origin to point ({}, {}) is {}",
        p_f64.x(),
        p_f64.y(),
        p_f64.distance_from_origin()
    );
}

// the type of field in generic struct can be changed, voila...
struct UnstablePoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> UnstablePoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: UnstablePoint<X2, Y2>) -> UnstablePoint<X1, Y2> {
        UnstablePoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn try_mixup_things() {
    let p1 = UnstablePoint { x: 5, y: 10.4 };
    let p2 = UnstablePoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x={}, p3.y={}", p3.x, p3.y);
}

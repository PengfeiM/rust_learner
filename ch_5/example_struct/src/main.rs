fn main() {
    // let's start with a example, which calculate the area of a rectangle.
    let width1 = 30;
    let height1 = 50;
    // as you can see, we define a function to take width and height to calculate area.
    println!(
        "The area of rectangle [30, 50] is {}",
        area(width1, height1)
    );
    // it's a little bit hard to remember so much individual vars.

    // of course, we can use tuple to unite the two dimensions of a rectangle.
    let rect1 = (30, 50);
    println!(
        "The area of rectangle [tuple({}, {})] is {}",
        rect1.0,
        rect1.1,
        area_tuple(rect1)
    );
    // by using tuple, we couple width and height together, but no one knows who's who.

    // let's make it better with struct.
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of rectangle [struct: {:?}] is {}",
        rect2,
        area_struct(&rect2)
    );
    // with struct, we have a clear view of what happened above.
    // with a struct #[derive(Debug)], we can also use dbg! to print it pretty.
    dbg!(&rect2);

    // there is still one problem we haven't solve, the fn area_struct can only be used to
    // calculate area for struct Rectangle.
    // but the struct and the fn is not related, which we will discuss in the next section.
}

/// area
/// calculate area with given width and height of a rectangle.
/// but the problem is that the params are not related clearly.
fn area(width: u32, height: u32) -> u32 {
    width * height
}

/// area_tuple
/// a better implement, using tuple as param.
/// this one is better, but it introduce another problem, no one knows which element in tuple is
/// width and height.
fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

/// Rectangle
/// a rectangle struct.
/// with #[derive(Debug)], the struct will implement std:fmt:Display automatically.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// area_struct
/// calculate the area of a rectangle implemented by a struct.
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

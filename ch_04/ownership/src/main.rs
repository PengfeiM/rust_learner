fn main() {
    let s = "hello"; // a string s declared in main.
    println!("{s}");

    new_scope();
    mem_alloc();
    ownership_function();
    return_value_scope();
}

fn new_scope() {
    // you can not access s here, s is invalid here, since s hasn't been declared in this
    // scope.
    // println!("{s}");
    /*
     * if you build with statement above, you will get:
     *  Compiling ownership v0.1.0 (/home/revan_m/codes/rust_begginer/rust_learner/ch_4/ownership)
     * error[E0425]: cannot find value `s` in this scope
     *  --> src/main.rs:10:16
     *   |
     *10 |     println!("{s}");
     *   |                ^ not found in this scope
     */

    let s = "hello"; // a new s, make s is valid from this point forward
    println!("{s}"); // now, it's fine to use String s
}

fn mem_alloc() {
    alloc_on_heap();
    data_move();
    scope_assign();
    clone_on_stack();
}
fn alloc_on_heap() {
    // string s above: the size of it can be determined when compiling.
    // it's not mutable, as it's stored on stack.
    // now we make a string in heap, it's mutable.
    let mut s_heap = String::from("hello");
    s_heap.push_str(", world");
    println!("{s_heap}");
}

fn data_move() {
    // String on heap would be a pointer point to the real addr on heap.
    // so, if you just do s2 = s1, s2 would be another pointer point to the same string as s1.
    // s1 -> "hello" <- s2
    let s1 = String::from("hello");
    let s2 = s1; // after s2 point to data, s1 is deprecated. we call it "move"
    // println!("{s1}, world!");
    /*
     * error[E0382]: borrow of moved value: `s1`
     *      --> src/main.rs:37:16
     *       |
     *    35 |     let s1 = String::from("hello");
     *       |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
     *    36 |     let s2 = s1; // after s2 point to data, s1 is deprecated.
     *       |              -- value moved here
     *    37 |     println!("{s1}, world!");
     *       |                ^^ value borrowed here after move
     *       |
     *       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
     *    help: consider cloning the value if the performance cost is acceptable
     *       |
     *    36 |     let s2 = s1.clone(); // after s2 point to data, s1 is deprecated.
     *       |                ++++++++
     * */
    println!("{s2}, world!");

    // if we wish to make s2 the same as s1, and not affect s1, we can use clone
    try_clone_str();
}

fn try_clone_str() {
    let s1 = String::from("try clone");
    let s2 = s1.clone();
    println!("s1: {s1}, world!");
    println!("s2: {s2}, world!");
}

fn scope_assign() {
    let mut s = String::from("hello");
    println!("original s: {s}");
    s = String::from("no hello");
    /*
     * by assigning a new value to s, Rust will allocate a new mem, and release the old one.
     * */
    println!("new s: {s}");
}

fn clone_on_stack() {
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}

fn ownership_function() {
    let s = String::from("hello");
    takes_ownership_by_move(s);
    // println!("s: {s}");
    /*
     * error[E0382]: borrow of moved value: `s`
     *   --> src/main.rs:100:19
     *    |
     * 98 |     let s = String::from("hello");
     *    |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
     * 99 |     takes_ownership_by_move(s);
     *    |                             - value moved here
     *100 |     println!("s: {s}");
     *    |                   ^ value borrowed here after move
     * */

    let x = 5;
    makes_copy(x);
}

fn takes_ownership_by_move(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn return_value_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its  return value into s1

    let s2 = String::from("hello");
    let s3 = takes_ownership_by_move(s2); // s2 -> fn takes_ownership_by_move.a_string -> s3
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

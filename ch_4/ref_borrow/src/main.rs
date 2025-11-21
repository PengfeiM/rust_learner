/*
* when you see some error report, just ignore the line number.
* */
fn main() {
    let s1 = String::from("hello");

    // try the ugly one.
    let (s1, len) = calculate_length_ugly(s1);
    println!("The length of '{s1}' is {len}.");
    /*
     * the example above is not so elegant.
     * it use a return value to give back ownership to the original owner, which is so unconvenient
     * that no one likes it. besides, it's also not so beautiful to return something we don't and
     * shouldn't care in this function.
     */

    // Luckily, we have reference in Rust.
    // with reference, we can borrow ownerships and return it elegantly.
    let s2 = String::from("hello, too");
    let len = calculate_length_elegantly(&s2);
    println!("The length of '{s2}' is {len}");
    /*
     * May noticed that we use &String instead of String.
     * & stands for reference, which allow us to refer to some value without taking its ownership.
     * it's easy  to see the benefits of reference:
     * - 1st: we don't need redundant code to transfer ownership.
     * - 2nd: the function can focus on its purpose without caring ownerships.
     *
     * but, How it's done?
     * let's see the data structure here.
     * &s2
     * name value
     * ptr -------> s2
     *              name     value       index   value
     *              ptr ---------------> 0       h
     *              len      10          1       e
     *              capacity 10          2       l
     *                                   3       l
     *                                   4       o
     *                                   5       ,
     *                                   6
     *                                   7       t
     *                                   8       o
     *                                   9       o
     *
     * In Rust, we call the action of creating a reference as **borrowing**.
     * */

    // let's try to modify a String with reference
    // let s3 = String::from("hello");
    // try_modify(&s3);
    // can not modify a immutable String.

    // mutable reference
    let mut s3 = String::from("hello");
    try_modify_with_mut_ref(&mut s3);
    println!("The value of s3 is: {s3}");

    // to make mutable reference safe, there is a big restriction of it.
    // there can be only one mutable reference to a mutable value!!!
    // let mut s4 = String::from("hello");
    // let r1 = &mut s4;
    // let r2 = &mut s4;
    // println!("{r1}, {r2}");
    /* you may get this error.
     * error[E0499]: cannot borrow `s4` as mutable more than once at a time
     *  --> src/main.rs:60:14
     *   |
     *59 |     let r1 = &mut s4;
     *   |              ------- first mutable borrow occurs here
     *60 |     let r2 = &mut s4;
     *   |              ^^^^^^^ second mutable borrow occurs here
     *61 |     println!("{r1}, {r2}");
     *   |                -- first borrow later used here
     * */
    // but r1's borrowing ends when the code goes out of r1's scope.
    // see example bellow.
    let mut s4 = String::from("hello");
    {
        let _r1 = &mut s4;
    } // r1 goes out of scope here, so we can make a new reference after.
    let r2 = &mut s4;
    println!("{r2}");

    // but there can be multi immutable reference.
    // of course, no mutable reference right after immutable reference
    let mut s5 = String::from("5. hello");

    let r1_s5 = &s5; // no problem
    let r2_s5 = &s5; //no problem
    // let r3_s5 = &mut s5; // BIG PROBLEM
    println!("{r1_s5}, {r2_s5}");
    // in the other way, after the immutable reference is used for the last time, we can create a
    // mutable reference.
    let r3_s5 = &mut s5; // no problem
    println!("{r3_s5}");
    // do not do this, cause it will make the scope of immutable reference r1_s5 and r2_s5 last here.
    // println!("{r1_s5}, {r2_s5}");

    // consider of the feature scope in Rust, we can create something like wild pointer in other
    // language, called Dangling Reference.
    let dangling_ref = dangle();
    /*
     * error[E0106]: missing lifetime specifier
     *  --> src/main.rs:165:16
     *    |
     *165 | fn dangle() -> &String {
     *    |                ^ expected named lifetime parameter
     *    |
     *    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
     *help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
     *    |
     *165 | fn dangle() -> &'static String {
     *    |                 +++++++
     *help: instead, you are more likely to want to return an owned value
     *    |
     *165 - fn dangle() -> &String {
     *165 + fn dangle() -> String {
     *    |
     * */
}

/// calculate_length_ugly
/// calculate the length of a String with giving back ownership.
/// # params:
/// - `s`: a String
///
/// # returns:
/// - String, a String that used to give back ownership to param s
/// - usize, the length of the String
fn calculate_length_ugly(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// calculate_length_elegantly
// calculate length os a String with reference, so we don't need to use a return value to give
// ownership back.
//
// # params:
// - `s`: a reference of String.
//
// # returns:
// - usize: the length os the String.
fn calculate_length_elegantly(s: &String) -> usize {
    s.len()
}

// try_modify
// try to modify a immutable String.
//
// # params:
// - `s`: a reference of String.
/*
fn try_modify(s: &String) {
    // you may get this error when compile
    /*
     * error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
     *  --> src/main.rs:84:5
     *   |
     *84 |     s.push_str(", world");
     *   |     ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
     *   |
     *help: consider changing this to be a mutable reference
     *   |
     *83 | fn try_modify(s: &mut String) {
     *   |                   +++
     *
     */
    println!("The value of s in try_modify is: {s}");
    // s.push_str(", world");
    println!("The value of s in try_modify is: {s}");
}
*/

fn try_modify_with_mut_ref(s: &mut String) {
    println!("The value of s in try_modify is: {s}");
    s.push_str(", world");
    println!("The value of s in try_modify is: {s}");
}

fn dangle() -> &String {
    let s = String::from("hello, dangling");
    return &s;
}

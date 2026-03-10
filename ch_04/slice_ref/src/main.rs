// slice is a reference
// so you can't return it.
// and it has some features of reference
fn main() {
    let mut s1 = String::from("hello world");

    let first_word = first_word_len(&s1); // 5
    println!("first word length: {first_word}");
    s1.clear(); // clear will make String s be empty, equals to ""
    println!("first word length: {first_word}");
    // we can see that var first_word_len gets out of sync with data s, which makes no error in
    // compiling, but leave a potential threat if you try to access something in s with
    // first_word_len.

    // to avoid this issue, Rust has string slice to fix it.
    let mut s2 = String::from("hello world");
    let first_word = first_word_slice(&s2);
    println!("first word slice: {first_word}");
    s2.clear();
    // can not borrow immutable here.
    // println!("first word slice: {first_word_slice}");
    // so, as we can see, &str is immutable reference, for example:
    // let s = "Hello, World!";

    // now, let's see how to use string slice as paramters.
    let my_string_literal = "hello world";
    let first_word = first_word_literal(my_string_literal);
    println!("first_word_literal: {first_word}");
    let first_word = first_word_literal(&my_string_literal[2..6]);
    println!("first_word_literal: {first_word}");
    let first_word = first_word_literal(&my_string_literal[..]);
    println!("first_word_literal: {first_word}");

    // do something about other slice.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // current char is ' ', return index
            return i;
        }
    }

    // no space detected, return total length of string.
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    /* you can make a string slice with syntax below:
     * let s = String::from("hello")
     * let slice = &s[0..2];
     * let slice = &s[..2];
     * let len = s.len()
     * let slice = &s[3..len];
     * let slice = &s[3..];
     * let slice = &s[0..len];
     * let slice = &s[..];
     * */

    // now, let's do some slices.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // no space, return it all.
    &s[..]
}

fn first_word_literal(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

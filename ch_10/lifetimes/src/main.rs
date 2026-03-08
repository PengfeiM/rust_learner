fn main() {
    println!("Hello, world!");
    try_lifetime();
    struct_lifetime();
    life_time_elision();

    // there is a special lifetime called static lifetime.
    // which makes the reference live for the entire duration of the program.
    {
        let s: &'static str = "I have a static lifetime";
        println!("{s}");
    }
}

fn try_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}!");
}

/// longest: test lifetime.
/// # longest
/// ## Why we use lifetime?
///    Compiling lifetimes v0.1.0 (/home/revan_m/codes/rust_begginer/rust_learner/ch_10/lifetimes)
///error[E0106]: missing lifetime specifier
///  --> src/main.rs:12:33
///   |
///12 | fn longest(x: &str, y: &str) -> &str {
///   |               ----     ----     ^ expected named lifetime parameter
///   |
///   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
///help: consider introducing a named lifetime parameter
///   |
///12 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
///   |           ++++     ++          ++          ++
///
///For more information about this error, try `rustc --explain E0106`.
///error: could not compile `lifetimes` (bin "lifetimes") due to 1 previous error
///
/// fn longest(x: &str, y: &str) -> &str {
///     if x.len() > y.len() { x } else { y }
/// }
///
/// ## how long will the return value last?
/// the return value share the scope of the parameter with lifetime whose scope ends first.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// longest_single_lifetime
/// as you see, if y is un related to the return value, y won't get a lifetime.
fn longest_single_lifetime<'a>(x: &'a str, y: &str) -> &'a str {
    println!("{y}");
    x
}

///longest_invalid_lifetime
///error[E0515]: cannot return value referencing local variable `result`
///  --> src/main.rs:54:5
///   |
///54 |     result.as_str()
///   |     ------^^^^^^^^^
///   |     |
///   |     returns a value referencing data owned by the current function
///   |     `result` is borrowed here
///fn longest_invalid_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
///    let result = String::from("really long string!");
///    result.as_str()
///}
fn longest_invalid_lifetime() {}

/// let's try some different struct, which hold fields that are references.
/// In this case, we would need to add lifetime on all the references in the struct.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_lifetime() {
    println!("struct can have lifetime,...");

    let novel = String::from("Call me Samuel. Some years ago ...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The first sentence of the novel: {}", i.part);
}

/// Here is a sugar in Rust, you don't need to write lifetime marker like `'a` if the relationship
/// of input lifetime and output lifetime is clear.
/// In early age of Rust, you must write the full line like:
/// ```rust
/// fn first_word<'a>(s: &'a str) -> &'a str{}
/// ```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn life_time_elision() {
    println!(
        "The first world in 'hello world' is {}",
        first_word("hello world")
    );
}

impl<'a> ImportantExcerpt<'a> {
    // 3 rules to see if lifetime is needed to be declared.

    /// Rule 2: lifetime elision with only one parameter.
    fn level(&self) -> i16 {
        3
    }

    /// Rule 1: assign a lifetime to every parameter.
    /// Rule 3: if the function is a method, the lifetime of `self` is assigned to all output
    /// lifetimes.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attension please: {announcement}");
        self.part
    }
}

fn main() {
    println!("Hello, world!");
    println!("================================================================");
    create_new_string();
    println!("================================================================");
    update_string();
    println!("================================================================");
    slice_string();
    println!("================================================================");
    iter_string();
    println!("================================================================");
}

fn create_new_string() {
    println!("create String...");
    // similarly, you can use new just as vector to create a string.
    // let mut s = String::new();

    // or else, you can create a string with given value.
    let data = "initial contents from var: data";
    let s = data.to_string();
    println!("String s: {s}");
    // it's the same even if you use literal directly.
    let s = "initial contents from raw literal".to_string();
    println!("String s: {s}");
    // as we have seen before, from() is also a good way.
    let s = String::from("initial contents with from()");
    println!("String s: {s}");

    // any utf-8 character can be put in String.
    let hello = String::from("السلام عليكم");
    println!("String: {hello}");
    let hello = String::from("Dobrý den");
    println!("String: {hello}");
    let hello = String::from("Hello");
    println!("String: {hello}");
    let hello = String::from("שלום");
    println!("String: {hello}");
    let hello = String::from("नमस्ते");
    println!("String: {hello}");
    let hello = String::from("こんにちは");
    println!("String: {hello}");
    let hello = String::from("안녕하세요");
    println!("String: {hello}");
    let hello = String::from("你好");
    println!("String: {hello}");
    let hello = String::from("Olá");
    println!("String: {hello}");
    let hello = String::from("Здравствуйте");
    println!("String: {hello}");
    let hello = String::from("Hola");
    println!("String: {hello}");
}

fn update_string() {
    println!("update String");
    // String can be changed with mutable size just like a vector.
    append_string();
    // String can be concatenating with '+' or format.
    plus_format_string();
}

fn append_string() {
    println!("update string by appending");
    // add string after the original string with push_str().
    let mut s = String::from("foo");
    println!("String s: {s}");
    s.push_str("bar");
    println!("String: {s}");

    let mut s1 = String::from("foo1");
    println!("String s1: {s1}");
    // use &str, push_str() won't take ownership of s2, s3.
    let s2 = "bar2";
    s1.push_str(s2);
    println!("String s1: {s1}");
    println!("String s2: {s2}");
    let s3 = String::from("bar3");
    s1.push_str(s3.as_str());
    println!("String s3: {s3}");

    // push_str for appending str, push for appending character.
    let mut s4 = String::from("I'm leading string, followed by ");
    println!("String s4: {s4}");
    let c = 'l';
    s4.push(c);
    println!("String s4: {s4}");
}

fn plus_format_string() {
    let s1 = String::from("pen");
    println!("I have a {s1}");
    let s2 = String::from("apple");
    println!("I have a {s2}");
    let s3 = s1 + &s2;
    println!("Ah, {s3}");
    // as s1's ownership has been moved, you can't access s1 again.
    // println!("I don't own {s1} any more");
    println!("I still have {s2}");

    // when we have multiple strings to concatenate, it would be unconvenient.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // like:
    // let s = s1 + "-" + s2 + "-" + "s3"
    // there is a macro format! to simplify the job.
    let s = format!("{s1}-{s2}-{s3}");
    println!("jointed string: {s}");
}

fn slice_string() {
    let hello = String::from("Здравствуйте");
    // let s = &hello[0..3];
    // this will cause a panic.
    // thread 'main' (36022564) panicked at src/main.rs:112:19:
    // byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let s = &hello[0..4];
    println!("a string slice: {s}");
}

fn iter_string() {
    let hello = String::from("Здравствуйте");
    println!("iterate string {hello}");
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
}

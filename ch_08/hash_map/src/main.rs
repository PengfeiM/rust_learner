/// finally, we come to the last collection, Hash Map.
/// similar to other language, hash map is a structure to save <k,v> pairs.
use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    println!("========================================");
    create_hashmap();
    println!("========================================");
    read_hashmap();
    println!("========================================");
    hashmap_kv_ownership();
    println!("========================================");
    update_hashmap();
    println!("========================================");
}

fn create_hashmap() {
    let mut scores = HashMap::new();
    println!("Here comes a empty hashmap: {scores:?}");

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Put something into the hashmap: {scores:?}");
}

fn read_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Here is a hashmap: {scores:?}");

    println!("Pick up one k-v pair in hashmap");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // copied turn &Option<T> -> Option<T>
    // unwrap_or unwrap Option<T> -> T or default value you provided.
    println!("The Blue team has obtained {score} scores");

    println!("Let's go through the hashmap");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn hashmap_kv_ownership() {
    // hashmap_kv_ownership
    // for types implement the `Copy` trait, like i32, the values are copied into the hashmap.
    // but for something like String, the values will be moved and hashmap will hold the
    // ownership.

    let key = String::from("Red");
    //  --- move occurs because `key` has type `String`, which does not implement the `Copy` trait
    let value = 30;

    let mut scores = HashMap::new();
    scores.insert(key, value);
    //            --- value moved here
    // println!("Read String after insert into hashmap: {key}");
    //                                                   ^^^ value borrowed here after move
    println!("Read i32 after insert into hashmap: {value}");
}

fn update_hashmap() {
    println!("try some way to update a hashmap");
    println!("---------------------------------");
    overwrite_value();
    println!("---------------------------------");
    add_kv_pair_only_if_key_absent();
    println!("---------------------------------");
    update_value_based_on_old_value();
    println!("---------------------------------");
}

fn overwrite_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("Original scores: {scores:?}");
    scores.insert(String::from("Blue"), 25);
    println!("New scores: {scores:?}");
}

fn add_kv_pair_only_if_key_absent() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("Original scores: {scores:?}");

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("New scores: {scores:?}");
}

fn update_value_based_on_old_value() {
    let text = "hell world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // let count = map.entry(word).or_insert(0);
        // *count += 1;
        *map.entry(word).or_insert(0) += 1;
    }

    println!("{map:?}");
}

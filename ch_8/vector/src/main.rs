fn main() {
    println!("Hello, world!");

    create_an_empty_vector();
    create_a_vector_with_macro();
    update_vector();
    read_a_vector();
    iterate_a_vector();
    vector_with_enum();
}

fn create_an_empty_vector() {
    // you may create a Vec with Vec::new()
    let v1: Vec<i32> = Vec::new();
    println!("An empty vector: {v1:?}");
}

fn create_a_vector_with_macro() {
    // besides, there are a macro you can use to create a vector
    let v2 = vec![1, 2, 3]; // Rust will recognize the type inside the vector automatically
    println!("A vector: {v2:?}");
}

fn update_vector() {
    let mut v = vec![1, 2, 3, 4];

    println!("A vector: {v:?}");
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("A updated vector: {v:?}");
}

fn read_a_vector() {
    let v = vec![1, 2, 3, 4, 5];

    // reading the element with index.
    let third: &i32 = &v[2];
    println!("The third element of {v:?} is {third}");

    // reading the element with get().
    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no fourth element"),
    }
    let sixth: Option<&i32> = v.get(5);
    match sixth {
        Some(sixth) => println!("The sixth element is {sixth}"),
        None => println!("There is no sixth element"),
    }
}

fn iterate_a_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v_mut = vec![100, 32, 57];
    for i in &mut v_mut {
        *i += 50;
    }
    for i in &v_mut {
        println!("{i}");
    }
}

fn vector_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.14),
    ];
    println!("enum-based vector: {row:?}");
}

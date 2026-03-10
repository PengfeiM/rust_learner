fn main() {
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    println!("Hello, world!");
    println!("===============================================================");
    // just_panic_it();
    index_out_of_bounds();
    println!("===============================================================");
}

fn just_panic_it() {
    // active panic
    panic!("crash and burn");
}

fn index_out_of_bounds() {
    // run into some default panic
    let v = vec![1, 2, 3];
    v[99];
}

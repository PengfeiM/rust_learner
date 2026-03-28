use test_org::add_two;

mod common;

#[test]
fn it_add_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}

#[test]
fn it_add_two_too() {
    common::setup();
    let result = add_two(3);
    assert_eq!(result, 5);
}

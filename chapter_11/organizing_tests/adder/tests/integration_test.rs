use adder::add;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    let result = add(2, 2);
    assert_eq!(result, 4);
}

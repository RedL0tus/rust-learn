extern crate test_organization;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(test_organization::add_two(2), 4);
}

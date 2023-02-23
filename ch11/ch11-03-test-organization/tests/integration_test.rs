extern crate ch11_03_test_organization;

mod common;

// tests directory doesn't need #[cfg(test)]
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, ch11_03_test_organization::add_two(2));
}

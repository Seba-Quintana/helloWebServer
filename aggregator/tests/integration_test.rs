use aggregator;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, aggregator::add_two(2));
}

use tests; // this refers to the crate name, not the directory holding this file

mod common; // can use module file for tests like this

#[test]
fn it_adds_two() {
  let value = common::returns_two();
  assert_eq!(4, tests::add_two(value));
}

use task_307::*;

#[test]
fn public_examples() {
    assert_eq!(count_separators_bang("cat"), "0");
    assert_eq!(count_separators_bang("!hi!"), "2");
}

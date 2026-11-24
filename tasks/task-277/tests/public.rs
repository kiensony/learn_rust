use task_277::*;

#[test]
fn public_examples() {
    assert_eq!(count_separators_star("cat"), "0");
    assert_eq!(count_separators_star("*hi*"), "2");
}

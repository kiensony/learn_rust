use task_287::*;

#[test]
fn public_examples() {
    assert_eq!(count_separators_tilde("cat"), "0");
    assert_eq!(count_separators_tilde("~hi~"), "2");
}

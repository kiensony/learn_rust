use task_283::*;

#[test]
fn public_examples() {
    assert_eq!(double_a_symbol_tilde("cat"), "cat");
    assert_eq!(double_a_symbol_tilde("~hi~"), "~~hi~~");
}

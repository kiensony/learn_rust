use task_293::*;

#[test]
fn public_examples() {
    assert_eq!(double_a_symbol_plus("cat"), "cat");
    assert_eq!(double_a_symbol_plus("+hi+"), "++hi++");
}

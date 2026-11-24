use task_273::*;

#[test]
fn public_examples() {
    assert_eq!(double_a_symbol_star("cat"), "cat");
    assert_eq!(double_a_symbol_star("*hi*"), "**hi**");
}

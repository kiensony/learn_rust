use task_303::*;

#[test]
fn public_examples() {
    assert_eq!(double_a_symbol_bang("cat"), "cat");
    assert_eq!(double_a_symbol_bang("!hi!"), "!!hi!!");
}

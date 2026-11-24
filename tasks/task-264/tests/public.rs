use task_264::*;

#[test]
fn public_examples() {
    assert_eq!(double_a_symbol_hash("cat"), "cat");
    assert_eq!(double_a_symbol_hash("#hi#"), "##hi##");
}

use task_272::*;

#[test]
fn public_examples() {
    assert_eq!(boundary_trim_star("cat"), "cat");
    assert_eq!(boundary_trim_star("*hi*"), "hi");
}

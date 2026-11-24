use task_263::*;

#[test]
fn public_examples() {
    assert_eq!(boundary_trim_hash("cat"), "cat");
    assert_eq!(boundary_trim_hash("#hi#"), "hi");
}

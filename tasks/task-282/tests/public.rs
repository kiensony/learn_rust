use task_282::*;

#[test]
fn public_examples() {
    assert_eq!(boundary_trim_tilde("cat"), "cat");
    assert_eq!(boundary_trim_tilde("~hi~"), "hi");
}

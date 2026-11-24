use task_281::*;

#[test]
fn public_examples() {
    assert_eq!(remove_a_suffix_tilde("cat"), "cat");
    assert_eq!(remove_a_suffix_tilde("~hi~"), "~hi");
}

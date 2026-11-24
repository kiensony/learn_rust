use task_2740::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_reverse(""), vec![]);
    assert_eq!(cli_rust_sources_reverse("a.txt"), vec![]);
}

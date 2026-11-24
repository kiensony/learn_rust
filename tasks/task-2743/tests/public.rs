use task_2743::*;

#[test]
fn public_examples() {
    assert_eq!(cli_stems_bounds(""), None);
    assert_eq!(cli_stems_bounds("a.txt"), Some((1u32, 1u32)));
}

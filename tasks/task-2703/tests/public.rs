use task_2703::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_bounds(""), None);
    assert_eq!(cli_basenames_bounds("a.txt"), Some((5u32, 5u32)));
}

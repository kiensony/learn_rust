use task_2710::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_reverse(""), vec![]);
    assert_eq!(cli_basenames_reverse("a.txt"), vec![5u32]);
}

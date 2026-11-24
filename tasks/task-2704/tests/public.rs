use task_2704::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_peaks(""), vec![]);
    assert_eq!(cli_basenames_peaks("a.txt"), vec![5u32]);
}

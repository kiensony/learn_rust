use task_2204::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_prefixes_peaks(""), vec![]);
    assert_eq!(borrowed_prefixes_peaks("a"), vec![1u32]);
}

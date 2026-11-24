use task_2246::*;

#[test]
fn public_examples() {
    assert_eq!(
        borrowed_prefixes_threshold_crossing("a b c\n\r\nup\tdown", 0),
        Some((0, 1))
    );
    assert_eq!(borrowed_prefixes_threshold_crossing("", 0), None);
}

use task_2246::*;

#[test]
fn transfer_case() {
    assert_eq!(
        borrowed_prefixes_threshold_crossing("a b c\n\r\nup\tdown a b c\n\r\nup\tdown", 1),
        Some((0, 1))
    );
}

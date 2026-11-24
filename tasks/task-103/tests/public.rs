use task_103::*;

#[test]
fn public_examples() {
    assert_eq!(complete_rows_10(65535, 4), 0);
    assert_eq!(complete_rows_10(13, 14), 1);
}

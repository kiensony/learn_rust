use task_2284::*;

#[test]
fn public_examples() {
    assert_eq!(find_newlines(&[]), 0);
    assert_eq!(find_newlines(b"a\nb"), 1);
}

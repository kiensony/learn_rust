use task_484::*;

#[test]
fn public_examples() {
    assert_eq!(window(&[1, 2, 3, 4], 1, 2), Some(&[2, 3][..]));
    assert_eq!(window(&[7, 8, 9], 2, 1), Some(&[9][..]));
}

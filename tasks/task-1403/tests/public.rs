use task_1403::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_bounds(&[]), None);
    assert_eq!(millisecond_remainder_bounds(&[4]), Some((68u32, 68u32)));
}

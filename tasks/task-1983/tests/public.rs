use task_1983::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_bounds(&[]), None);
    assert_eq!(atomic_record_high_bounds(&[4]), Some((0u32, 0u32)));
}

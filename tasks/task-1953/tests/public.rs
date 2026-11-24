use task_1953::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_bounds(&[]), None);
    assert_eq!(atomic_running_total_bounds(&[4]), Some((0u32, 0u32)));
}

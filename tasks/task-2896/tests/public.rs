use task_2896::*;

#[test]
fn public_examples() {
    assert_eq!(
        little_endian_registers_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 1539))
    );
    assert_eq!(little_endian_registers_threshold_crossing(&[], 0), None);
}

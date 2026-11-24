use task_1998::*;

#[test]
fn public_examples() {
    assert_eq!(
        atomic_toggle_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![
            0u64, 3u64, 5u64, 12u64, 0u64, 21u64, 26u64, 26u64, 0u64, 8u64
        ]
    );
    assert_eq!(atomic_toggle_rolling_load(&[], 0), vec![]);
}

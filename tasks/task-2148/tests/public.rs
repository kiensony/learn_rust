use task_2148::*;

#[test]
fn public_examples() {
    assert_eq!(
        queue_rotation_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![
            6u64, 9u64, 12u64, 21u64, 15u64, 0u64, 26u64, 8u64, 31u64, 3u64
        ]
    );
    assert_eq!(queue_rotation_rolling_load(&[], 0), vec![]);
}

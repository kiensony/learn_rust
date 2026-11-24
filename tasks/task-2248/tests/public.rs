use task_2248::*;

#[test]
fn public_examples() {
    assert_eq!(
        borrowed_scalars_rolling_load("a\nblue\nblue\r\nz\n", 1),
        vec![
            0u64, 1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64, 11u64, 12u64, 13u64,
            14u64
        ]
    );
    assert_eq!(borrowed_scalars_rolling_load("", 0), vec![]);
}

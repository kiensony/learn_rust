use task_2248::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_scalars_rolling_load("a", 1), vec![0u64]);
    assert_eq!(
        borrowed_scalars_rolling_load("a\nblue\nblue\r\nz\n", 2),
        vec![
            1u64, 3u64, 5u64, 7u64, 9u64, 11u64, 13u64, 15u64, 17u64, 19u64, 21u64, 23u64, 25u64,
            27u64
        ]
    );
    assert_eq!(borrowed_scalars_rolling_load("   x\té\n\ncat\n", 0), vec![]);
    assert_eq!(
        borrowed_scalars_rolling_load("nine\nq\nthree\nx\n", 100),
        vec![]
    );
    assert_eq!(
        borrowed_scalars_rolling_load("a b c\n\r\nup\tdown", 3),
        vec![
            3u64, 6u64, 9u64, 12u64, 15u64, 18u64, 21u64, 24u64, 27u64, 30u64, 33u64, 36u64, 39u64
        ]
    );
    assert_eq!(
        borrowed_scalars_rolling_load("\n\nzz\n", 1),
        vec![0u64, 1u64, 2u64, 3u64, 4u64]
    );
    assert_eq!(
        borrowed_scalars_rolling_load("a\nblue\nblue\r\nz\n", 2),
        vec![
            1u64, 3u64, 5u64, 7u64, 9u64, 11u64, 13u64, 15u64, 17u64, 19u64, 21u64, 23u64, 25u64,
            27u64
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2248::TRACK_PROOF, 0x52540000 | 2248);
}

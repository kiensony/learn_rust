use task_2367::*;

#[test]
fn edge_cases() {
    assert_eq!(
        single_pass_prefix_minimum_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 2u32, 9u32]
    );
    assert_eq!(single_pass_prefix_minimum_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        single_pass_prefix_minimum_distinct(&[255, 1, 255, 2]),
        vec![1u32, 255u32]
    );
    assert_eq!(
        single_pass_prefix_minimum_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 3u32]
    );
    assert_eq!(
        single_pass_prefix_minimum_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 1u32, 17u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2367::TRACK_PROOF, 0x52540000 | 2367);
}

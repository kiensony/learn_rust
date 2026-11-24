use task_2191::*;

#[test]
fn edge_cases() {
    assert_eq!(
        duplicate_insertion_cost_trace(&[9, 2, 2, 8, 0, 5]),
        vec![1u32, 2u32, 2u32, 3u32, 4u32, 5u32]
    );
    assert_eq!(
        duplicate_insertion_cost_trace(&[0, 0, 0]),
        vec![1u32, 1u32, 1u32]
    );
    assert_eq!(
        duplicate_insertion_cost_trace(&[255, 1, 255, 2]),
        vec![1u32, 2u32, 2u32, 3u32]
    );
    assert_eq!(
        duplicate_insertion_cost_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32]
    );
    assert_eq!(
        duplicate_insertion_cost_trace(&[17, 17, 1, 0, 2]),
        vec![1u32, 1u32, 2u32, 3u32, 4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2191::TRACK_PROOF, 0x52540000 | 2191);
}

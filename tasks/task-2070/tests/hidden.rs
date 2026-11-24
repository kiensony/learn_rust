use task_2070::*;

#[test]
fn edge_cases() {
    assert_eq!(
        double_buffer_receiver_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 2u32, 9u32]
    );
    assert_eq!(double_buffer_receiver_reverse(&[0, 0, 0]), vec![]);
    assert_eq!(
        double_buffer_receiver_reverse(&[255, 1, 255, 2]),
        vec![1u32, 255u32]
    );
    assert_eq!(
        double_buffer_receiver_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![26u32, 6u32, 3u32]
    );
    assert_eq!(
        double_buffer_receiver_reverse(&[17, 17, 1, 0, 2]),
        vec![2u32, 17u32, 17u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2070::TRACK_PROOF, 0x52540000 | 2070);
}

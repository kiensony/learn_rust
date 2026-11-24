use task_2661::*;

#[test]
fn edge_cases() {
    assert_eq!(
        lending_first_byte_trace(&[9, 2, 2, 8, 0, 5]),
        vec![9u32, 8u32]
    );
    assert_eq!(lending_first_byte_trace(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        lending_first_byte_trace(&[255, 1, 255, 2]),
        vec![255u32, 2u32]
    );
    assert_eq!(
        lending_first_byte_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 12u32, 0u32, 31u32]
    );
    assert_eq!(
        lending_first_byte_trace(&[17, 17, 1, 0, 2]),
        vec![17u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2661::TRACK_PROOF, 0x52540000 | 2661);
}

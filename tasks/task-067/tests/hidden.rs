use task_067::*;

#[test]
fn edge_cases() {
    assert_eq!(reverse_a_reading_2(0), 32767);
    assert_eq!(reverse_a_reading_2(1), 32767);
    assert_eq!(reverse_a_reading_2(1), 32767);
    assert_eq!(reverse_a_reading_2(2), 32766);
    assert_eq!(reverse_a_reading_2(3), 32766);
    assert_eq!(reverse_a_reading_2(65535), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_067::TRACK_PROOF, 0x52540000 | 67);
}

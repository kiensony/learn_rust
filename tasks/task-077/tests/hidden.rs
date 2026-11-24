use task_077::*;

#[test]
fn edge_cases() {
    assert_eq!(reverse_a_reading_3(0, 6), 21843);
    assert_eq!(reverse_a_reading_3(1, 9), 21841);
    assert_eq!(reverse_a_reading_3(2, 12), 21840);
    assert_eq!(reverse_a_reading_3(3, 15), 21839);
    assert_eq!(reverse_a_reading_3(4, 1), 21843);
    assert_eq!(reverse_a_reading_3(65535, 4), 0);
    assert_eq!(reverse_a_reading_3(65535, 1), 0);
    assert_eq!(reverse_a_reading_3(0, 0), 21845);
    assert_eq!(reverse_a_reading_3(1, 0), 21844);
    assert_eq!(reverse_a_reading_3(2, 0), 21844);
    assert_eq!(reverse_a_reading_3(3, 0), 21844);
    assert_eq!(reverse_a_reading_3(5, 0), 21843);
    assert_eq!(reverse_a_reading_3(6, 0), 21843);
    assert_eq!(reverse_a_reading_3(7, 0), 21842);
    assert_eq!(reverse_a_reading_3(20, 0), 21838);
    assert_eq!(reverse_a_reading_3(100, 0), 21811);
}

#[test]
fn submission_marker() {
    assert_eq!(task_077::TRACK_PROOF, 0x52540000 | 77);
}

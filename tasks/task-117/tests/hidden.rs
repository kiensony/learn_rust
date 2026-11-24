use task_117::*;

#[test]
fn edge_cases() {
    assert_eq!(fit_a_byte_2(0), 2);
    assert_eq!(fit_a_byte_2(1), 3);
    assert_eq!(fit_a_byte_2(1), 3);
    assert_eq!(fit_a_byte_2(2), 4);
    assert_eq!(fit_a_byte_2(3), 5);
    assert_eq!(fit_a_byte_2(65535), 255);
    assert_eq!(fit_a_byte_2(20), 22);
    assert_eq!(fit_a_byte_2(40), 42);
    assert_eq!(fit_a_byte_2(50), 52);
    assert_eq!(fit_a_byte_2(100), 102);
    assert_eq!(fit_a_byte_2(101), 103);
    assert_eq!(fit_a_byte_2(255), 255);
    assert_eq!(fit_a_byte_2(256), 255);
}

#[test]
fn submission_marker() {
    assert_eq!(task_117::TRACK_PROOF, 0x52540000 | 117);
}

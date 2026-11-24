use task_127::*;

#[test]
fn edge_cases() {
    assert_eq!(fit_a_byte_3(0, 6), 9);
    assert_eq!(fit_a_byte_3(1, 9), 13);
    assert_eq!(fit_a_byte_3(2, 12), 17);
    assert_eq!(fit_a_byte_3(3, 15), 21);
    assert_eq!(fit_a_byte_3(4, 1), 8);
    assert_eq!(fit_a_byte_3(65535, 4), 255);
    assert_eq!(fit_a_byte_3(20, 7), 30);
    assert_eq!(fit_a_byte_3(40, 10), 53);
    assert_eq!(fit_a_byte_3(50, 13), 66);
    assert_eq!(fit_a_byte_3(100, 16), 119);
    assert_eq!(fit_a_byte_3(101, 2), 106);
    assert_eq!(fit_a_byte_3(255, 5), 255);
    assert_eq!(fit_a_byte_3(256, 8), 255);
    assert_eq!(fit_a_byte_3(65535, 1), 255);
    assert_eq!(fit_a_byte_3(0, 0), 3);
    assert_eq!(fit_a_byte_3(1, 0), 4);
    assert_eq!(fit_a_byte_3(2, 0), 5);
    assert_eq!(fit_a_byte_3(3, 0), 6);
    assert_eq!(fit_a_byte_3(5, 0), 8);
    assert_eq!(fit_a_byte_3(6, 0), 9);
    assert_eq!(fit_a_byte_3(7, 0), 10);
    assert_eq!(fit_a_byte_3(20, 0), 23);
    assert_eq!(fit_a_byte_3(100, 0), 103);
}

#[test]
fn submission_marker() {
    assert_eq!(task_127::TRACK_PROOF, 0x52540000 | 127);
}

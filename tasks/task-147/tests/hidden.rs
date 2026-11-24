use task_147::*;

#[test]
fn edge_cases() {
    assert_eq!(fit_a_byte_8(0, 2), 10);
    assert_eq!(fit_a_byte_8(1, 3), 10);
    assert_eq!(fit_a_byte_8(7, 4), 11);
    assert_eq!(fit_a_byte_8(8, 5), 11);
    assert_eq!(fit_a_byte_8(9, 6), 11);
    assert_eq!(fit_a_byte_8(65535, 0), 255);
    assert_eq!(fit_a_byte_8(20, 1), 27);
    assert_eq!(fit_a_byte_8(40, 2), 46);
    assert_eq!(fit_a_byte_8(50, 3), 55);
    assert_eq!(fit_a_byte_8(100, 4), 104);
    assert_eq!(fit_a_byte_8(101, 5), 104);
    assert_eq!(fit_a_byte_8(255, 6), 255);
    assert_eq!(fit_a_byte_8(256, 0), 255);
    assert_eq!(fit_a_byte_8(18, 3), 23);
    assert_eq!(fit_a_byte_8(3, 18), 23);
    assert_eq!(fit_a_byte_8(4, 4), 8);
    assert_eq!(fit_a_byte_8(0, 0), 8);
    assert_eq!(fit_a_byte_8(1, 0), 9);
    assert_eq!(fit_a_byte_8(7, 0), 15);
    assert_eq!(fit_a_byte_8(8, 0), 16);
    assert_eq!(fit_a_byte_8(9, 0), 17);
    assert_eq!(fit_a_byte_8(10, 0), 18);
    assert_eq!(fit_a_byte_8(11, 0), 19);
    assert_eq!(fit_a_byte_8(16, 0), 24);
    assert_eq!(fit_a_byte_8(17, 0), 25);
    assert_eq!(fit_a_byte_8(20, 0), 28);
    assert_eq!(fit_a_byte_8(100, 0), 108);
}

#[test]
fn submission_marker() {
    assert_eq!(task_147::TRACK_PROOF, 0x52540000 | 147);
}

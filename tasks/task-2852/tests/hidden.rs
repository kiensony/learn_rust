use task_2852::*;

#[test]
fn edge_cases() {
    assert_eq!(little_endian_registers_total(&[9, 2, 2, 8, 0, 5]), 3851);
    assert_eq!(little_endian_registers_total(&[0, 0, 0]), 0);
    assert_eq!(little_endian_registers_total(&[255, 1, 255, 2]), 1278);
    assert_eq!(
        little_endian_registers_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        23081
    );
    assert_eq!(little_endian_registers_total(&[17, 17, 1, 0, 2]), 4370);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2852::TRACK_PROOF, 0x52540000 | 2852);
}

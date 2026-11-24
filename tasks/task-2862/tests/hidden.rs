use task_2862::*;

#[test]
fn edge_cases() {
    assert_eq!(big_endian_registers_total(&[9, 2, 2, 8, 0, 5]), 2831);
    assert_eq!(big_endian_registers_total(&[0, 0, 0]), 0);
    assert_eq!(big_endian_registers_total(&[255, 1, 255, 2]), 130563);
    assert_eq!(
        big_endian_registers_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        10586
    );
    assert_eq!(big_endian_registers_total(&[17, 17, 1, 0, 2]), 4625);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2862::TRACK_PROOF, 0x52540000 | 2862);
}

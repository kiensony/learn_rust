use task_2870::*;

#[test]
fn edge_cases() {
    assert_eq!(
        big_endian_registers_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 520u32, 2306u32]
    );
    assert_eq!(big_endian_registers_reverse(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        big_endian_registers_reverse(&[255, 1, 255, 2]),
        vec![65282u32, 65281u32]
    );
    assert_eq!(
        big_endian_registers_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![2079u32, 26u32, 5391u32, 2316u32, 774u32]
    );
    assert_eq!(
        big_endian_registers_reverse(&[17, 17, 1, 0, 2]),
        vec![256u32, 4369u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2870::TRACK_PROOF, 0x52540000 | 2870);
}

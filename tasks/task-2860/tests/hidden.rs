use task_2860::*;

#[test]
fn edge_cases() {
    assert_eq!(
        little_endian_registers_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![1280u32, 2050u32, 521u32]
    );
    assert_eq!(little_endian_registers_reverse(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        little_endian_registers_reverse(&[255, 1, 255, 2]),
        vec![767u32, 511u32]
    );
    assert_eq!(
        little_endian_registers_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![7944u32, 6656u32, 3861u32, 3081u32, 1539u32]
    );
    assert_eq!(
        little_endian_registers_reverse(&[17, 17, 1, 0, 2]),
        vec![1u32, 4369u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2860::TRACK_PROOF, 0x52540000 | 2860);
}

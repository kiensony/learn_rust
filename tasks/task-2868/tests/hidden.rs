use task_2868::*;

#[test]
fn edge_cases() {
    assert_eq!(
        big_endian_registers_steps(&[9, 2, 2, 8, 0, 5]),
        vec![1786u32, 515u32]
    );
    assert_eq!(big_endian_registers_steps(&[0, 0, 0]), vec![]);
    assert_eq!(big_endian_registers_steps(&[255, 1, 255, 2]), vec![1u32]);
    assert_eq!(
        big_endian_registers_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![1542u32, 3075u32, 5365u32, 2053u32]
    );
    assert_eq!(
        big_endian_registers_steps(&[17, 17, 1, 0, 2]),
        vec![4113u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2868::TRACK_PROOF, 0x52540000 | 2868);
}

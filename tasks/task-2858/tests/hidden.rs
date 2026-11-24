use task_2858::*;

#[test]
fn edge_cases() {
    assert_eq!(
        little_endian_registers_steps(&[9, 2, 2, 8, 0, 5]),
        vec![1529u32, 770u32]
    );
    assert_eq!(little_endian_registers_steps(&[0, 0, 0]), vec![]);
    assert_eq!(
        little_endian_registers_steps(&[255, 1, 255, 2]),
        vec![256u32]
    );
    assert_eq!(
        little_endian_registers_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![1542u32, 780u32, 2795u32, 1288u32]
    );
    assert_eq!(
        little_endian_registers_steps(&[17, 17, 1, 0, 2]),
        vec![4368u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2858::TRACK_PROOF, 0x52540000 | 2858);
}

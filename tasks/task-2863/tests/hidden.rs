use task_2863::*;

#[test]
fn edge_cases() {
    assert_eq!(
        big_endian_registers_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((5u32, 2306u32))
    );
    assert_eq!(big_endian_registers_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        big_endian_registers_bounds(&[255, 1, 255, 2]),
        Some((65281u32, 65282u32))
    );
    assert_eq!(
        big_endian_registers_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((26u32, 5391u32))
    );
    assert_eq!(
        big_endian_registers_bounds(&[17, 17, 1, 0, 2]),
        Some((256u32, 4369u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2863::TRACK_PROOF, 0x52540000 | 2863);
}

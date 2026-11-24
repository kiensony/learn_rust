use task_2864::*;

#[test]
fn edge_cases() {
    assert_eq!(
        big_endian_registers_peaks(&[9, 2, 2, 8, 0, 5]),
        vec![2306u32]
    );
    assert_eq!(big_endian_registers_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        big_endian_registers_peaks(&[255, 1, 255, 2]),
        vec![65281u32, 65282u32]
    );
    assert_eq!(
        big_endian_registers_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![774u32, 2316u32, 5391u32]
    );
    assert_eq!(
        big_endian_registers_peaks(&[17, 17, 1, 0, 2]),
        vec![4369u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2864::TRACK_PROOF, 0x52540000 | 2864);
}

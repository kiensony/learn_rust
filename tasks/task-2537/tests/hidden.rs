use task_2537::*;

#[test]
fn edge_cases() {
    assert_eq!(
        abi_endian_swap_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 512u32, 1280u32, 2048u32, 2304u32]
    );
    assert_eq!(abi_endian_swap_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        abi_endian_swap_distinct(&[255, 1, 255, 2]),
        vec![256u32, 512u32, 65280u32]
    );
    assert_eq!(
        abi_endian_swap_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            0u32, 768u32, 1536u32, 2048u32, 2304u32, 3072u32, 3840u32, 5376u32, 6656u32, 7936u32
        ]
    );
    assert_eq!(
        abi_endian_swap_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 256u32, 512u32, 4352u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2537::TRACK_PROOF, 0x52540000 | 2537);
}

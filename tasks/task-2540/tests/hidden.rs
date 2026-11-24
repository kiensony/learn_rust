use task_2540::*;

#[test]
fn edge_cases() {
    assert_eq!(
        abi_endian_swap_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![1280u32, 0u32, 2048u32, 512u32, 512u32, 2304u32]
    );
    assert_eq!(abi_endian_swap_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        abi_endian_swap_reverse(&[255, 1, 255, 2]),
        vec![512u32, 65280u32, 256u32, 65280u32]
    );
    assert_eq!(
        abi_endian_swap_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            7936u32, 2048u32, 6656u32, 0u32, 3840u32, 5376u32, 3072u32, 2304u32, 1536u32, 768u32
        ]
    );
    assert_eq!(
        abi_endian_swap_reverse(&[17, 17, 1, 0, 2]),
        vec![512u32, 0u32, 256u32, 4352u32, 4352u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2540::TRACK_PROOF, 0x52540000 | 2540);
}

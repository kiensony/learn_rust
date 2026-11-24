use task_2538::*;

#[test]
fn edge_cases() {
    assert_eq!(
        abi_endian_swap_steps(&[9, 2, 2, 8, 0, 5]),
        vec![1792u32, 0u32, 1536u32, 2048u32, 1280u32]
    );
    assert_eq!(abi_endian_swap_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        abi_endian_swap_steps(&[255, 1, 255, 2]),
        vec![65024u32, 65024u32, 64768u32]
    );
    assert_eq!(
        abi_endian_swap_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            768u32, 768u32, 768u32, 2304u32, 1536u32, 3840u32, 6656u32, 4608u32, 5888u32
        ]
    );
    assert_eq!(
        abi_endian_swap_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 4096u32, 256u32, 512u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2538::TRACK_PROOF, 0x52540000 | 2538);
}

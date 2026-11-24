use task_2549::*;

#[test]
fn edge_cases() {
    assert_eq!(abi_endian_swap_frequent_readings(&[4], 0), vec![]);
    assert_eq!(
        abi_endian_swap_frequent_readings(&[9, 2, 2, 8, 0, 5], 1),
        vec![512u32]
    );
    assert_eq!(abi_endian_swap_frequent_readings(&[0, 0, 0], 2), vec![0u32]);
    assert_eq!(
        abi_endian_swap_frequent_readings(&[255, 1, 255, 2], 3),
        vec![65280u32, 256u32, 512u32]
    );
    assert_eq!(
        abi_endian_swap_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 256),
        vec![
            768u32, 1536u32, 2304u32, 3072u32, 5376u32, 3840u32, 0u32, 6656u32, 2048u32, 7936u32
        ]
    );
    assert_eq!(
        abi_endian_swap_frequent_readings(&[17, 17, 1, 0, 2], 0),
        vec![]
    );
    assert_eq!(
        abi_endian_swap_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![768u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2549::TRACK_PROOF, 0x52540000 | 2549);
}

use task_2549::*;

#[test]
fn public_examples() {
    assert_eq!(
        abi_endian_swap_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![768u32, 1536u32, 2304u32]
    );
    assert_eq!(abi_endian_swap_frequent_readings(&[], 0), vec![]);
}

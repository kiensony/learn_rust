use task_2473::*;

#[test]
fn public_examples() {
    assert_eq!(strided_raw_reads_bounds(&[]), None);
    assert_eq!(strided_raw_reads_bounds(&[4]), Some((4u32, 4u32)));
}

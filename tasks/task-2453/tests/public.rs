use task_2453::*;

#[test]
fn public_examples() {
    assert_eq!(forward_raw_reads_bounds(&[]), None);
    assert_eq!(forward_raw_reads_bounds(&[4]), Some((4u32, 4u32)));
}

use task_1243::*;

#[test]
fn public_examples() {
    assert_eq!(limited_reads_bounds(&[]), None);
    assert_eq!(limited_reads_bounds(&[4]), Some((4u32, 4u32)));
}

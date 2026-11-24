use task_2443::*;

#[test]
fn public_examples() {
    assert_eq!(page_alignment_bounds(&[]), None);
    assert_eq!(page_alignment_bounds(&[4]), Some((32u32, 32u32)));
}

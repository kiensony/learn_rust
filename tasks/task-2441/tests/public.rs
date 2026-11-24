use task_2441::*;

#[test]
fn public_examples() {
    assert_eq!(page_alignment_trace(&[]), vec![]);
    assert_eq!(page_alignment_trace(&[4]), vec![32u32]);
}

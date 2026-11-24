use task_2271::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_trace(&[]), vec![]);
    assert_eq!(exclusive_hotspots_trace(&[4]), vec![4u32]);
}

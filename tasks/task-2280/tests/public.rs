use task_2280::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_reverse(&[]), vec![]);
    assert_eq!(exclusive_hotspots_reverse(&[4]), vec![4u32]);
}

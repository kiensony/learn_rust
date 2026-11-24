use task_2277::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_distinct(&[]), vec![]);
    assert_eq!(exclusive_hotspots_distinct(&[4]), vec![4u32]);
}

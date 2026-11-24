use task_2275::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_changes(&[]), vec![]);
    assert_eq!(exclusive_hotspots_changes(&[4]), vec![4u32]);
}

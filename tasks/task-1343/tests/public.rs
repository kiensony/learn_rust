use task_1343::*;

#[test]
fn public_examples() {
    assert_eq!(packed_booleans_bounds(""), None);
    assert_eq!(packed_booleans_bounds("1"), Some((1u32, 1u32)));
}

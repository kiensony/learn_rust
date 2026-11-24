use task_1483::*;

#[test]
fn public_examples() {
    assert_eq!(response_boundaries_bounds(""), None);
    assert_eq!(response_boundaries_bounds("200"), Some((0u32, 0u32)));
}

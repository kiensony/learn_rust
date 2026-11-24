use task_2223::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_scalars_bounds(""), None);
    assert_eq!(borrowed_scalars_bounds("a"), Some((0u32, 0u32)));
}

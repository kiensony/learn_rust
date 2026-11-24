use task_2203::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_prefixes_bounds(""), None);
    assert_eq!(borrowed_prefixes_bounds("a"), Some((1u32, 1u32)));
}

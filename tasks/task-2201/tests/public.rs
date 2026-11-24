use task_2201::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_prefixes_trace(""), vec![]);
    assert_eq!(borrowed_prefixes_trace("a"), vec![1u32]);
}

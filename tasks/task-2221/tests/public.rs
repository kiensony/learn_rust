use task_2221::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_scalars_trace(""), vec![]);
    assert_eq!(borrowed_scalars_trace("a"), vec![0u32]);
}

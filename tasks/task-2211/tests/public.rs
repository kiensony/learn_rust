use task_2211::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_offsets_trace(""), vec![]);
    assert_eq!(borrowed_offsets_trace("a"), vec![]);
}

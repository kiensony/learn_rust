use task_2227::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_scalars_distinct(""), vec![]);
    assert_eq!(borrowed_scalars_distinct("a"), vec![0u32]);
}

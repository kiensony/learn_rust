use task_2230::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_scalars_reverse(""), vec![]);
    assert_eq!(borrowed_scalars_reverse("a"), vec![0u32]);
}

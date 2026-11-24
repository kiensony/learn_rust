use task_2210::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_prefixes_reverse(""), vec![]);
    assert_eq!(borrowed_prefixes_reverse("a"), vec![1u32]);
}

use task_1490::*;

#[test]
fn public_examples() {
    assert_eq!(response_boundaries_reverse(""), vec![]);
    assert_eq!(response_boundaries_reverse("200"), vec![0u32]);
}

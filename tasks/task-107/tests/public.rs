use task_107::*;

#[test]
fn public_examples() {
    assert_eq!(label_size("  lamp"), 4);
    assert_eq!(label_size("box "), 3);
}

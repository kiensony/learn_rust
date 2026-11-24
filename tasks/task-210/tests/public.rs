use task_210::*;

#[test]
fn public_examples() {
    assert_eq!(knocks(1), "!");
    assert_eq!(knocks(4), "!!!!");
}

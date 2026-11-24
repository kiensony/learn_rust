use task_305::*;

#[test]
fn public_examples() {
    assert_eq!(needs_update("  blue", "blue"), false);
    assert_eq!(needs_update("East", "east"), true);
}

use task_257::*;

#[test]
fn public_examples() {
    assert_eq!(cycle_ticks_10(255, 4), 4);
    assert_eq!(cycle_ticks_10(13, 14), 10);
}

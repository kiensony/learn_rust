use task_537::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_pipe("first\n123"), 3);
    assert_eq!(shortest_field_with_first_tie_pipe("header\n猫|123"), 1);
}

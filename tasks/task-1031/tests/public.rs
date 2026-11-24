use task_1031::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(["first\n123", "header\n猫|123"]),
        [3, 3]
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe([
            String::from("header\n猫|123"),
            String::from("hi|kit")
        ]),
        [3, 2]
    );
}

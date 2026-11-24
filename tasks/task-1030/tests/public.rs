use task_1030::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe(["first\n123", "header\n猫|123"]),
        [3, 3]
    );
    assert_eq!(longest_field_with_last_tie_pipe(["hi|kit"]), [3]);
}

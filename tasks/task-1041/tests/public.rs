use task_1041::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals(["72;12", "猫=12;note"]),
        [2, 2]
    );
    assert_eq!(shortest_field_with_first_tie_equals(["hi=kit"]), [2]);
}

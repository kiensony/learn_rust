use task_1038::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_equals(["72;12", "猫=12;note"]),
        [0, 3]
    );
    assert_eq!(
        prefix_before_final_mark_equals([String::from("猫=12;note"), String::from("hi=kit")]),
        [3, 2]
    );
}

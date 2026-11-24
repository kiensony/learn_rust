use task_988::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_equals(vec!["72;12", "猫=12;note"]),
        vec![0usize, 3usize]
    );
    assert_eq!(
        prefix_before_final_mark_equals(vec!["猫=12;note", "hi=kit"]),
        vec![3usize, 2usize]
    );
}

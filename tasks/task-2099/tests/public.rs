use task_2099::*;

#[test]
fn public_examples() {
    assert_eq!(
        four_slot_intake_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![3u32, 6u32, 9u32]
    );
    assert_eq!(four_slot_intake_frequent_readings(&[], 0), vec![]);
}

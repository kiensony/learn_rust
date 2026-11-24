use task_1349::*;

#[test]
fn public_examples() {
    assert_eq!(
        field_indexes_frequent_readings("0,0,0", 3),
        vec![0u32, 1u32, 2u32]
    );
    assert_eq!(field_indexes_frequent_readings("", 0), vec![]);
}

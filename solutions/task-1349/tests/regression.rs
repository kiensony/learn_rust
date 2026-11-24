use task_1349::*;

#[test]
fn transfer_case() {
    assert_eq!(
        field_indexes_frequent_readings("0,0,0 0,0,0", 2),
        vec![0u32, 1u32]
    );
}

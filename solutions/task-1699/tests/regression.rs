use task_1699::*;

#[test]
fn transfer_case() {
    assert_eq!(
        preview_version_frequent_readings("1.0.0 1.2.0 2.0.0 1.0.0 1.2.0 2.0.0", 2),
        vec![0u32]
    );
}

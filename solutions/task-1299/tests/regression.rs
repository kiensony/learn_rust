use task_1299::*;

#[test]
fn transfer_case() {
    assert_eq!(
        option_terminator_frequent_readings("-- --level=4 x -- --level=4 x", 2),
        vec![9u32, 1u32]
    );
}

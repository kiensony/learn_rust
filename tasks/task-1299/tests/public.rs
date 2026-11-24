use task_1299::*;

#[test]
fn public_examples() {
    assert_eq!(
        option_terminator_frequent_readings("-- --level=4 x", 3),
        vec![9u32, 1u32]
    );
    assert_eq!(option_terminator_frequent_readings("", 0), vec![]);
}

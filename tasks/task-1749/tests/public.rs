use task_1749::*;

#[test]
fn public_examples() {
    assert_eq!(
        population_macro_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![2u32, 3u32, 4u32]
    );
    assert_eq!(population_macro_frequent_readings(&[], 0), vec![]);
}

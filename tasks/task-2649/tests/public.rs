use task_2649::*;

#[test]
fn public_examples() {
    assert_eq!(
        embedded_saturation_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![6u32, 12u32, 18u32]
    );
    assert_eq!(embedded_saturation_frequent_readings(&[], 0), vec![]);
}

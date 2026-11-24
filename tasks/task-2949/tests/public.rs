use task_2949::*;

#[test]
fn public_examples() {
    assert_eq!(
        stability_band_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![0u32]
    );
    assert_eq!(stability_band_frequent_readings(&[], 0), vec![]);
}

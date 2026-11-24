use task_203::*;

#[test]
fn public_examples() {
    assert_eq!(alarm_band_10(65535, 4), true);
    assert_eq!(alarm_band_10(10, 11), false);
}

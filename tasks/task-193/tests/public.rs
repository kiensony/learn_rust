use task_193::*;

#[test]
fn public_examples() {
    assert_eq!(alarm_band_8(4, 0), true);
    assert_eq!(alarm_band_8(10, 1), false);
}

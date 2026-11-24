use task_173::*;

#[test]
fn public_examples() {
    assert_eq!(alarm_band_3(4, 0), false);
    assert_eq!(alarm_band_3(13, 3), true);
}

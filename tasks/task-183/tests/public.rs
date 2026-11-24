use task_183::*;

#[test]
fn public_examples() {
    assert_eq!(alarm_band_5(4, 0), true);
    assert_eq!(alarm_band_5(6, 0), false);
}

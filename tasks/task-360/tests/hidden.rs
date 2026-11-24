use task_360::*;

#[test]
fn boundaries() {
    assert_eq!(letter_count(&"".into()), 0);
    assert_eq!(letter_count(&"雪☀".into()), 2);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 360);
}

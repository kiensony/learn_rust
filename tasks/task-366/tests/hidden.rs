use task_366::*;

#[test]
fn boundaries() {
    assert_eq!(first_reading(&[-6]), -6);
    assert_eq!(first_reading(&[2, 5]), 2);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 366);
}

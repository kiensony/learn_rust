use task_855::*;

#[test]
fn boundaries() {
    assert_eq!(price(17), 4);
    assert_eq!(scale::quarter(0), 0);
    assert_eq!(scale::quarter(3), 0);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 855);
}

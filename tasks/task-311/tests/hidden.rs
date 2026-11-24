use task_311::*;

#[test]
fn boundaries() {
    assert_eq!(tag("".into()), ("".into(), 0));
    assert_eq!(tag("雪".into()), ("雪".into(), 3));
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 311);
}

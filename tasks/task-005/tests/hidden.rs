use task_005::{longest, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 5);
}

#[test]
fn tie_returns_first() {
    assert_eq!(longest("xy", "ab"), "xy");
}

#[test]
fn empty() {
    assert_eq!(longest("", "x"), "x");
    assert_eq!(longest("", ""), "");
}

#[test]
fn same_ptr_ok() {
    let s = "same";
    assert_eq!(longest(s, s), s);
}

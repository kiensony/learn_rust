use task_004::{label, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 4);
}

#[test]
fn empty_name() {
    assert_eq!(label(String::from(""), 0), "0:");
}

#[test]
fn already_upper_and_digits() {
    assert_eq!(label(String::from("R2d2"), 99), "99:R2D2");
}

#[test]
fn non_ascii_untouched_except_ascii() {
    assert_eq!(label(String::from("ñada"), 1), "1:ñADA");
}

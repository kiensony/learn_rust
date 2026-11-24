use task_560::*;

#[test]
fn boundaries() {
    assert_eq!(title("", "new"), "new");
    assert_eq!(title("", ""), "");
    assert_eq!(
        {
            let s = String::from("own");
            title(&s, "x").as_ptr() == s.as_ptr()
        },
        true
    );
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 560);
}

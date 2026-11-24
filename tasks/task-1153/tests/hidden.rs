use task_1153::*;

#[test]
fn boundaries() {
    assert_eq!(prefix("雪"), "雪");
    assert_eq!(prefix(":z"), "");
    assert_eq!(
        {
            let s = String::from("x:y");
            prefix(&s).as_ptr() == s.as_ptr()
        },
        true
    );
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1153);
}

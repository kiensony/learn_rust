use task_720::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash(Some("hi/kit")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("/é/a")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("/")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("cat")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("//one/猫/ab")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("aa/bb")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("ok/🐈")), None);
    assert_eq!(
        prefix_before_final_mark_slash(Some("path/first/last")),
        None
    );
    assert_eq!(prefix_before_final_mark_slash(Some("path/path/")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("path/")), None);
    assert_eq!(prefix_before_final_mark_slash(None), None);
    assert_eq!(prefix_before_final_mark_slash(Some("3/72")), Some(3));
    assert_eq!(prefix_before_final_mark_slash(Some("/9")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("3300")), None);
    assert_eq!(prefix_before_final_mark_slash(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_720::TRACK_PROOF, 0x52540000 | 720);
}

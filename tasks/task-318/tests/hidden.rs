use task_318::*;

#[test]
fn edge_cases() {
    assert_eq!(unpack((String::from(""), 0)), (0, String::from("")));
    assert_eq!(
        unpack((String::from("λ"), 65535)),
        (65535, String::from("λ"))
    );
    assert_eq!(
        unpack((String::from(" tea "), 1)),
        (1, String::from(" tea "))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_318::TRACK_PROOF, 0x52540000 | 318);
}

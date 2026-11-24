use task_1061::*;

#[test]
fn edge_cases() {
    assert_eq!(key_of(&Record(false, true)), &false);
    assert_eq!(key_of(&Record(0u16, 0u16)), &0);
    assert_eq!(
        key_of(&Record(String::from("λ"), String::from(""))),
        &String::from("λ")
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1061::TRACK_PROOF, 0x52540000 | 1061);
}

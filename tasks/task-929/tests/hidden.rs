use task_929::*;

#[test]
fn edge_cases() {
    assert_eq!(
        provide(Some(String::new()), || panic!("unused fallback")),
        ""
    );
    assert_eq!(provide(None, || String::new()), "");
    assert_eq!(
        provide(None, {
            let owned = String::from("move");
            move || owned
        }),
        "move"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_929::TRACK_PROOF, 0x52540000 | 929);
}

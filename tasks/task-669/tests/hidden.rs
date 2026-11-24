use task_669::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("hi/kit")),
        Some(6)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("/é/a")),
        Some(4)
    );
    assert_eq!(marks_removed_at_both_ends_slash(Event::Data("")), Some(0));
    assert_eq!(marks_removed_at_both_ends_slash(Event::Data("/")), Some(0));
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("cat")),
        Some(3)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("//one/猫/ab")),
        Some(10)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("aa/bb")),
        Some(5)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("ok/🐈")),
        Some(7)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("path/first/last")),
        Some(10)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("path/path/")),
        Some(4)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(Event::Data("path/")),
        Some(0)
    );
    assert_eq!(marks_removed_at_both_ends_slash(Event::Heartbeat), Some(0));
    assert_eq!(marks_removed_at_both_ends_slash(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_669::TRACK_PROOF, 0x52540000 | 669);
}

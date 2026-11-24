use task_673::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("hi/kit")),
        Some(3)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("/é/a")),
        Some(2)
    );
    assert_eq!(longest_field_with_last_tie_slash(Event::Data("")), Some(0));
    assert_eq!(longest_field_with_last_tie_slash(Event::Data("/")), Some(0));
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("cat")),
        Some(3)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("//one/猫/ab")),
        Some(3)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("aa/bb")),
        Some(2)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("ok/🐈")),
        Some(4)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("path/first/last")),
        Some(5)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("path/path/")),
        Some(4)
    );
    assert_eq!(
        longest_field_with_last_tie_slash(Event::Data("path/")),
        Some(0)
    );
    assert_eq!(longest_field_with_last_tie_slash(Event::Heartbeat), Some(0));
    assert_eq!(longest_field_with_last_tie_slash(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_673::TRACK_PROOF, 0x52540000 | 673);
}

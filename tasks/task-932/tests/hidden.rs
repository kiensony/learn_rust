use task_932::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_pipe("hi|kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("|é|a", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(shortest_field_with_first_tie_pipe("|", |s| s.len() * 2), 0);
    assert_eq!(
        shortest_field_with_first_tie_pipe("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("||one|猫|ab", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("aa|bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("ok|🐈", |s| s.len() * 2),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\nleft|right\r\n", |s| s.chars().count() + 1),
        5
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\n\n", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("猫|x\nend|a", |s| s.chars().count() + 1),
        2
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_932::TRACK_PROOF, 0x52540000 | 932);
}

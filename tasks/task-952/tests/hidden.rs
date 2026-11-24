use task_952::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_hash("unclosed[cat", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(shortest_field_with_first_tie_hash("[]", |s| s.len() * 2), 0);
    assert_eq!(
        shortest_field_with_first_tie_hash("[42]", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("[321]", |s| s.len() * 2),
        6
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("hi#kit", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("#é#a", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(shortest_field_with_first_tie_hash("#", |s| s.len() * 2), 0);
    assert_eq!(
        shortest_field_with_first_tie_hash("cat", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("##one#猫#ab", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("aa#bb", |s| s.chars().count() + 1),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_952::TRACK_PROOF, 0x52540000 | 952);
}

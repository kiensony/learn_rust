use task_1200::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_hash("unclosed[cat", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_hash("[]", "backup"), "backup");
    assert_eq!(shortest_field_with_first_tie_hash("[42]", "backup"), "42");
    assert_eq!(shortest_field_with_first_tie_hash("[321]", "backup"), "321");
    assert_eq!(
        shortest_field_with_first_tie_hash("hi#kit", "backup"),
        "backup"
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("#é#a", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_hash("", "backup"), "backup");
    assert_eq!(shortest_field_with_first_tie_hash("#", "backup"), "backup");
    assert_eq!(
        shortest_field_with_first_tie_hash("cat", "backup"),
        "backup"
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("##one#猫#ab", "backup"),
        "backup"
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("aa#bb", "backup"),
        "backup"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1200::TRACK_PROOF, 0x52540000 | 1200);
}

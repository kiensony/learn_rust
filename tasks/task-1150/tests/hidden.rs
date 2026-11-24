use task_1150::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_hash(&"unclosed[cat"), 0);
    assert_eq!(shortest_field_with_first_tie_hash(&String::from("[]")), 0);
    assert_eq!(shortest_field_with_first_tie_hash(&"[42]"), 2);
    assert_eq!(
        shortest_field_with_first_tie_hash(&String::from("[321]")),
        3
    );
    assert_eq!(shortest_field_with_first_tie_hash(&"hi#kit"), 0);
    assert_eq!(shortest_field_with_first_tie_hash(&String::from("#é#a")), 0);
    assert_eq!(shortest_field_with_first_tie_hash(&""), 0);
    assert_eq!(shortest_field_with_first_tie_hash(&String::from("#")), 0);
    assert_eq!(shortest_field_with_first_tie_hash(&"cat"), 0);
    assert_eq!(
        shortest_field_with_first_tie_hash(&String::from("##one#猫#ab")),
        0
    );
    assert_eq!(shortest_field_with_first_tie_hash(&"aa#bb"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1150::TRACK_PROOF, 0x52540000 | 1150);
}

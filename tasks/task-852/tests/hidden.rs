use task_852::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_hash("unclosed[cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("[]"),
        Err(FieldError::Missing)
    );
    assert_eq!(shortest_field_with_first_tie_hash("[42]"), Ok(2));
    assert_eq!(shortest_field_with_first_tie_hash("[321]"), Ok(3));
    assert_eq!(
        shortest_field_with_first_tie_hash("hi#kit"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("#é#a"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("#"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("##one#猫#ab"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("aa#bb"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("ok#🐈"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_852::TRACK_PROOF, 0x52540000 | 852);
}

use task_157::*;

#[test]
fn edge_cases() {
    assert_eq!(milliseconds(0), Some(0));
    assert_eq!(milliseconds(4294967), Some(4294967000));
    assert_eq!(milliseconds(4294968), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_157::TRACK_PROOF, 0x52540000 | 157);
}

use task_2998::*;

#[test]
fn edge_cases() {
    assert_eq!(read_versioned(&[3]), Err("length"));
    assert_eq!(read_versioned(&[0, 0, 0]), Err("version"));
    assert_eq!(read_versioned(&[2, 255, 0]), Ok(65280));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2998::TRACK_PROOF, 0x52540000 | 2998);
}

use task_1048::*;

#[test]
fn edge_cases() {
    assert_eq!(zip_array([0u8; 0], [false; 0]), []);
    assert_eq!(zip_array([1, 1], [false, true]), [(1, false), (1, true)]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1048::TRACK_PROOF, 0x52540000 | 1048);
}

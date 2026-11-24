use task_2102::*;

#[test]
fn edge_cases() {
    assert_eq!(sorted_keys_total(&[9, 2, 2, 8, 0, 5]), 24);
    assert_eq!(sorted_keys_total(&[0, 0, 0]), 0);
    assert_eq!(sorted_keys_total(&[255, 1, 255, 2]), 258);
    assert_eq!(sorted_keys_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]), 131);
    assert_eq!(sorted_keys_total(&[17, 17, 1, 0, 2]), 20);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2102::TRACK_PROOF, 0x52540000 | 2102);
}

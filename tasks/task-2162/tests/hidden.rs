use task_2162::*;

#[test]
fn edge_cases() {
    assert_eq!(sorted_insertion_moves_total(&[9, 2, 2, 8, 0, 5]), 9);
    assert_eq!(sorted_insertion_moves_total(&[0, 0, 0]), 0);
    assert_eq!(sorted_insertion_moves_total(&[255, 1, 255, 2]), 3);
    assert_eq!(
        sorted_insertion_moves_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        12
    );
    assert_eq!(sorted_insertion_moves_total(&[17, 17, 1, 0, 2]), 7);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2162::TRACK_PROOF, 0x52540000 | 2162);
}

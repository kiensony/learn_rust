use task_2419::*;

#[test]
fn edge_cases() {
    assert_eq!(word_alignment_longest_run(&[9, 2, 2, 8, 0, 5]), 2);
    assert_eq!(word_alignment_longest_run(&[0, 0, 0]), 3);
    assert_eq!(word_alignment_longest_run(&[255, 1, 255, 2]), 1);
    assert_eq!(
        word_alignment_longest_run(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        2
    );
    assert_eq!(word_alignment_longest_run(&[17, 17, 1, 0, 2]), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2419::TRACK_PROOF, 0x52540000 | 2419);
}

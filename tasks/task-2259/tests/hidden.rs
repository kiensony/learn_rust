use task_2259::*;

#[test]
fn edge_cases() {
    assert_eq!(cpu_sample_counts_longest_run(&[9, 2, 2, 8, 0, 5]), 3);
    assert_eq!(cpu_sample_counts_longest_run(&[0, 0, 0]), 1);
    assert_eq!(cpu_sample_counts_longest_run(&[255, 1, 255, 2]), 2);
    assert_eq!(
        cpu_sample_counts_longest_run(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        10
    );
    assert_eq!(cpu_sample_counts_longest_run(&[17, 17, 1, 0, 2]), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2259::TRACK_PROOF, 0x52540000 | 2259);
}

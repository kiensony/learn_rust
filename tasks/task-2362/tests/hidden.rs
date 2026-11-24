use task_2362::*;

#[test]
fn edge_cases() {
    assert_eq!(single_pass_prefix_minimum_total(&[9, 2, 2, 8, 0, 5]), 15);
    assert_eq!(single_pass_prefix_minimum_total(&[0, 0, 0]), 0);
    assert_eq!(single_pass_prefix_minimum_total(&[255, 1, 255, 2]), 258);
    assert_eq!(
        single_pass_prefix_minimum_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        18
    );
    assert_eq!(single_pass_prefix_minimum_total(&[17, 17, 1, 0, 2]), 35);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2362::TRACK_PROOF, 0x52540000 | 2362);
}

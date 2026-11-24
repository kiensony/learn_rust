use task_913::*;

#[test]
fn edge_cases() {
    assert_eq!(visit_total(&[], |_| 1), 0);
    assert_eq!(visit_total(&[1], |_| u32::MAX), 4294967295);
    assert_eq!(
        {
            let mut calls = 0;
            let sum = visit_total(&[4, 4, 4], |_| {
                calls += 1;
                calls
            });
            (sum, calls)
        },
        (6, 3)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_913::TRACK_PROOF, 0x52540000 | 913);
}

use task_1443::*;

#[test]
fn edge_cases() {
    assert_eq!(
        deadline_slack_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((97u32, 250u32))
    );
    assert_eq!(deadline_slack_bounds(&[0, 0, 0]), Some((250u32, 250u32)));
    assert_eq!(
        deadline_slack_bounds(&[255, 1, 255, 2]),
        Some((216u32, 4085u32))
    );
    assert_eq!(
        deadline_slack_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((5u32, 277u32))
    );
    assert_eq!(
        deadline_slack_bounds(&[17, 17, 1, 0, 2]),
        Some((39u32, 250u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1443::TRACK_PROOF, 0x52540000 | 1443);
}

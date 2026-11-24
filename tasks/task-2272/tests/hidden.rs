use task_2272::*;

#[test]
fn edge_cases() {
    assert_eq!(exclusive_hotspots_total(&[9, 2, 2, 8, 0, 5]), 22);
    assert_eq!(exclusive_hotspots_total(&[0, 0, 0]), 0);
    assert_eq!(exclusive_hotspots_total(&[255, 1, 255, 2]), 3);
    assert_eq!(
        exclusive_hotspots_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        131
    );
    assert_eq!(exclusive_hotspots_total(&[17, 17, 1, 0, 2]), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2272::TRACK_PROOF, 0x52540000 | 2272);
}

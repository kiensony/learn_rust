use task_1812::*;

#[test]
fn edge_cases() {
    assert_eq!(shard_maxima_total(&[9, 2, 2, 8, 0, 5]), 17);
    assert_eq!(shard_maxima_total(&[0, 0, 0]), 0);
    assert_eq!(shard_maxima_total(&[255, 1, 255, 2]), 257);
    assert_eq!(shard_maxima_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]), 87);
    assert_eq!(shard_maxima_total(&[17, 17, 1, 0, 2]), 19);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1812::TRACK_PROOF, 0x52540000 | 1812);
}

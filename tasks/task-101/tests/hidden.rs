use task_101::*;

#[test]
fn edge_cases() {
    assert_eq!(count_packs_10(65535, 0), 0);
    assert_eq!(count_packs_10(1, 2), 10);
    assert_eq!(count_packs_10(65535, 9), 90);
    assert_eq!(count_packs_10(10, 11), 100);
    assert_eq!(count_packs_10(65535, 11), 110);
    assert_eq!(count_packs_10(65535, 65535), 655350);
    assert_eq!(count_packs_10(1, 0), 0);
    assert_eq!(count_packs_10(0, 1), 0);
    assert_eq!(count_packs_10(0, 0), 0);
    assert_eq!(count_packs_10(1, 1), 10);
    assert_eq!(count_packs_10(9, 9), 90);
    assert_eq!(count_packs_10(10, 10), 100);
    assert_eq!(count_packs_10(11, 11), 110);
    assert_eq!(count_packs_10(12, 12), 120);
    assert_eq!(count_packs_10(13, 13), 130);
    assert_eq!(count_packs_10(20, 20), 200);
    assert_eq!(count_packs_10(21, 21), 210);
    assert_eq!(count_packs_10(100, 100), 1000);
}

#[test]
fn submission_marker() {
    assert_eq!(task_101::TRACK_PROOF, 0x52540000 | 101);
}

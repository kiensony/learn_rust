use task_091::*;

#[test]
fn edge_cases() {
    assert_eq!(count_packs_8(0, 2), 16);
    assert_eq!(count_packs_8(1, 3), 16);
    assert_eq!(count_packs_8(7, 4), 24);
    assert_eq!(count_packs_8(8, 5), 24);
    assert_eq!(count_packs_8(9, 6), 24);
    assert_eq!(count_packs_8(65535, 0), 524280);
    assert_eq!(count_packs_8(18, 3), 120);
    assert_eq!(count_packs_8(3, 18), 120);
    assert_eq!(count_packs_8(4, 4), 0);
    assert_eq!(count_packs_8(0, 0), 0);
    assert_eq!(count_packs_8(1, 0), 8);
    assert_eq!(count_packs_8(7, 0), 56);
    assert_eq!(count_packs_8(8, 0), 64);
    assert_eq!(count_packs_8(9, 0), 72);
    assert_eq!(count_packs_8(10, 0), 80);
    assert_eq!(count_packs_8(11, 0), 88);
    assert_eq!(count_packs_8(16, 0), 128);
    assert_eq!(count_packs_8(17, 0), 136);
    assert_eq!(count_packs_8(20, 0), 160);
    assert_eq!(count_packs_8(100, 0), 800);
}

#[test]
fn submission_marker() {
    assert_eq!(task_091::TRACK_PROOF, 0x52540000 | 91);
}

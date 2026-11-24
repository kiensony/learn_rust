use task_081::*;

#[test]
fn edge_cases() {
    assert_eq!(count_packs_5(0, 2), 0);
    assert_eq!(count_packs_5(1, 0), 5);
    assert_eq!(count_packs_5(4, 1), 15);
    assert_eq!(count_packs_5(5, 2), 15);
    assert_eq!(count_packs_5(6, 0), 30);
    assert_eq!(count_packs_5(65535, 1), 327670);
    assert_eq!(count_packs_5(0, 9), 0);
    assert_eq!(count_packs_5(100, 0), 500);
    assert_eq!(count_packs_5(100, 1), 495);
    assert_eq!(count_packs_5(0, 0), 0);
    assert_eq!(count_packs_5(5, 0), 25);
    assert_eq!(count_packs_5(7, 0), 35);
    assert_eq!(count_packs_5(8, 0), 40);
    assert_eq!(count_packs_5(10, 0), 50);
    assert_eq!(count_packs_5(11, 0), 55);
    assert_eq!(count_packs_5(20, 0), 100);
}

#[test]
fn submission_marker() {
    assert_eq!(task_081::TRACK_PROOF, 0x52540000 | 81);
}

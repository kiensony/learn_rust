use task_151::*;

#[test]
fn edge_cases() {
    assert_eq!(whole_cartons_10(65535, 0), 0);
    assert_eq!(whole_cartons_10(1, 2), 1);
    assert_eq!(whole_cartons_10(65535, 9), 1);
    assert_eq!(whole_cartons_10(10, 11), 1);
    assert_eq!(whole_cartons_10(65535, 11), 2);
    assert_eq!(whole_cartons_10(65535, 65535), 6554);
    assert_eq!(whole_cartons_10(65535, 20), 2);
    assert_eq!(whole_cartons_10(40, 41), 4);
    assert_eq!(whole_cartons_10(65535, 50), 5);
    assert_eq!(whole_cartons_10(100, 101), 10);
    assert_eq!(whole_cartons_10(65535, 101), 11);
    assert_eq!(whole_cartons_10(255, 256), 26);
    assert_eq!(whole_cartons_10(65535, 256), 26);
    assert_eq!(whole_cartons_10(1, 0), 0);
    assert_eq!(whole_cartons_10(0, 1), 0);
    assert_eq!(whole_cartons_10(0, 0), 0);
    assert_eq!(whole_cartons_10(1, 1), 1);
    assert_eq!(whole_cartons_10(9, 9), 1);
    assert_eq!(whole_cartons_10(10, 10), 1);
    assert_eq!(whole_cartons_10(11, 11), 2);
    assert_eq!(whole_cartons_10(12, 12), 2);
    assert_eq!(whole_cartons_10(13, 13), 2);
    assert_eq!(whole_cartons_10(20, 20), 2);
    assert_eq!(whole_cartons_10(21, 21), 3);
    assert_eq!(whole_cartons_10(100, 100), 10);
}

#[test]
fn submission_marker() {
    assert_eq!(task_151::TRACK_PROOF, 0x52540000 | 151);
}

use task_141::*;

#[test]
fn edge_cases() {
    assert_eq!(whole_cartons_8(0, 2), 1);
    assert_eq!(whole_cartons_8(1, 3), 1);
    assert_eq!(whole_cartons_8(7, 4), 1);
    assert_eq!(whole_cartons_8(8, 5), 1);
    assert_eq!(whole_cartons_8(9, 6), 1);
    assert_eq!(whole_cartons_8(65535, 0), 8192);
    assert_eq!(whole_cartons_8(20, 1), 3);
    assert_eq!(whole_cartons_8(40, 2), 5);
    assert_eq!(whole_cartons_8(50, 3), 6);
    assert_eq!(whole_cartons_8(100, 4), 12);
    assert_eq!(whole_cartons_8(101, 5), 12);
    assert_eq!(whole_cartons_8(255, 6), 32);
    assert_eq!(whole_cartons_8(256, 0), 32);
    assert_eq!(whole_cartons_8(18, 3), 2);
    assert_eq!(whole_cartons_8(3, 18), 2);
    assert_eq!(whole_cartons_8(4, 4), 0);
    assert_eq!(whole_cartons_8(0, 0), 0);
    assert_eq!(whole_cartons_8(1, 0), 1);
    assert_eq!(whole_cartons_8(7, 0), 1);
    assert_eq!(whole_cartons_8(8, 0), 1);
    assert_eq!(whole_cartons_8(9, 0), 2);
    assert_eq!(whole_cartons_8(10, 0), 2);
    assert_eq!(whole_cartons_8(11, 0), 2);
    assert_eq!(whole_cartons_8(16, 0), 2);
    assert_eq!(whole_cartons_8(17, 0), 3);
    assert_eq!(whole_cartons_8(20, 0), 3);
    assert_eq!(whole_cartons_8(100, 0), 13);
}

#[test]
fn submission_marker() {
    assert_eq!(task_141::TRACK_PROOF, 0x52540000 | 141);
}

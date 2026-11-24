use task_131::*;

#[test]
fn edge_cases() {
    assert_eq!(whole_cartons_5(0, 2), 0);
    assert_eq!(whole_cartons_5(1, 0), 1);
    assert_eq!(whole_cartons_5(4, 1), 1);
    assert_eq!(whole_cartons_5(5, 2), 1);
    assert_eq!(whole_cartons_5(6, 0), 2);
    assert_eq!(whole_cartons_5(65535, 1), 13107);
    assert_eq!(whole_cartons_5(20, 2), 4);
    assert_eq!(whole_cartons_5(40, 0), 8);
    assert_eq!(whole_cartons_5(50, 1), 10);
    assert_eq!(whole_cartons_5(100, 2), 20);
    assert_eq!(whole_cartons_5(101, 0), 21);
    assert_eq!(whole_cartons_5(255, 1), 51);
    assert_eq!(whole_cartons_5(256, 2), 51);
    assert_eq!(whole_cartons_5(0, 9), 0);
    assert_eq!(whole_cartons_5(100, 0), 20);
    assert_eq!(whole_cartons_5(100, 1), 20);
    assert_eq!(whole_cartons_5(0, 0), 0);
    assert_eq!(whole_cartons_5(5, 0), 1);
    assert_eq!(whole_cartons_5(7, 0), 2);
    assert_eq!(whole_cartons_5(8, 0), 2);
    assert_eq!(whole_cartons_5(10, 0), 2);
    assert_eq!(whole_cartons_5(11, 0), 3);
    assert_eq!(whole_cartons_5(20, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_131::TRACK_PROOF, 0x52540000 | 131);
}

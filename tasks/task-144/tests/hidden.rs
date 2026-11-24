use task_144::*;

#[test]
fn edge_cases() {
    assert_eq!(round_up_8(0, 2), 8);
    assert_eq!(round_up_8(1, 3), 8);
    assert_eq!(round_up_8(7, 4), 8);
    assert_eq!(round_up_8(8, 5), 8);
    assert_eq!(round_up_8(9, 6), 8);
    assert_eq!(round_up_8(65535, 0), 65536);
    assert_eq!(round_up_8(20, 1), 24);
    assert_eq!(round_up_8(40, 2), 40);
    assert_eq!(round_up_8(50, 3), 48);
    assert_eq!(round_up_8(100, 4), 96);
    assert_eq!(round_up_8(101, 5), 96);
    assert_eq!(round_up_8(255, 6), 256);
    assert_eq!(round_up_8(256, 0), 256);
    assert_eq!(round_up_8(18, 3), 16);
    assert_eq!(round_up_8(3, 18), 16);
    assert_eq!(round_up_8(4, 4), 0);
    assert_eq!(round_up_8(0, 0), 0);
    assert_eq!(round_up_8(1, 0), 8);
    assert_eq!(round_up_8(7, 0), 8);
    assert_eq!(round_up_8(8, 0), 8);
    assert_eq!(round_up_8(9, 0), 16);
    assert_eq!(round_up_8(10, 0), 16);
    assert_eq!(round_up_8(11, 0), 16);
    assert_eq!(round_up_8(16, 0), 16);
    assert_eq!(round_up_8(17, 0), 24);
    assert_eq!(round_up_8(20, 0), 24);
    assert_eq!(round_up_8(100, 0), 104);
}

#[test]
fn submission_marker() {
    assert_eq!(task_144::TRACK_PROOF, 0x52540000 | 144);
}

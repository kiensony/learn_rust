use task_143::*;

#[test]
fn edge_cases() {
    assert_eq!(round_down_8(0, 2), 0);
    assert_eq!(round_down_8(1, 3), 0);
    assert_eq!(round_down_8(7, 4), 0);
    assert_eq!(round_down_8(8, 5), 0);
    assert_eq!(round_down_8(9, 6), 0);
    assert_eq!(round_down_8(65535, 0), 65528);
    assert_eq!(round_down_8(20, 1), 16);
    assert_eq!(round_down_8(40, 2), 32);
    assert_eq!(round_down_8(50, 3), 40);
    assert_eq!(round_down_8(100, 4), 96);
    assert_eq!(round_down_8(101, 5), 96);
    assert_eq!(round_down_8(255, 6), 248);
    assert_eq!(round_down_8(256, 0), 256);
    assert_eq!(round_down_8(18, 3), 8);
    assert_eq!(round_down_8(3, 18), 8);
    assert_eq!(round_down_8(4, 4), 0);
    assert_eq!(round_down_8(0, 0), 0);
    assert_eq!(round_down_8(1, 0), 0);
    assert_eq!(round_down_8(7, 0), 0);
    assert_eq!(round_down_8(8, 0), 8);
    assert_eq!(round_down_8(9, 0), 8);
    assert_eq!(round_down_8(10, 0), 8);
    assert_eq!(round_down_8(11, 0), 8);
    assert_eq!(round_down_8(16, 0), 16);
    assert_eq!(round_down_8(17, 0), 16);
    assert_eq!(round_down_8(20, 0), 16);
    assert_eq!(round_down_8(100, 0), 96);
}

#[test]
fn submission_marker() {
    assert_eq!(task_143::TRACK_PROOF, 0x52540000 | 143);
}

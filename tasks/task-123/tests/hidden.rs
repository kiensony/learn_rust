use task_123::*;

#[test]
fn edge_cases() {
    assert_eq!(round_down_3(0, 6), 6);
    assert_eq!(round_down_3(1, 9), 9);
    assert_eq!(round_down_3(2, 12), 12);
    assert_eq!(round_down_3(3, 15), 18);
    assert_eq!(round_down_3(4, 1), 3);
    assert_eq!(round_down_3(65535, 4), 65535);
    assert_eq!(round_down_3(20, 7), 27);
    assert_eq!(round_down_3(40, 10), 48);
    assert_eq!(round_down_3(50, 13), 63);
    assert_eq!(round_down_3(100, 16), 114);
    assert_eq!(round_down_3(101, 2), 102);
    assert_eq!(round_down_3(255, 5), 258);
    assert_eq!(round_down_3(256, 8), 264);
    assert_eq!(round_down_3(65535, 1), 65535);
    assert_eq!(round_down_3(0, 0), 0);
    assert_eq!(round_down_3(1, 0), 0);
    assert_eq!(round_down_3(2, 0), 0);
    assert_eq!(round_down_3(3, 0), 3);
    assert_eq!(round_down_3(5, 0), 3);
    assert_eq!(round_down_3(6, 0), 6);
    assert_eq!(round_down_3(7, 0), 6);
    assert_eq!(round_down_3(20, 0), 18);
    assert_eq!(round_down_3(100, 0), 99);
}

#[test]
fn submission_marker() {
    assert_eq!(task_123::TRACK_PROOF, 0x52540000 | 123);
}

use task_153::*;

#[test]
fn edge_cases() {
    assert_eq!(round_down_10(65535, 0), 0);
    assert_eq!(round_down_10(1, 2), 0);
    assert_eq!(round_down_10(65535, 9), 0);
    assert_eq!(round_down_10(10, 11), 10);
    assert_eq!(round_down_10(65535, 11), 10);
    assert_eq!(round_down_10(65535, 65535), 65530);
    assert_eq!(round_down_10(65535, 20), 20);
    assert_eq!(round_down_10(40, 41), 40);
    assert_eq!(round_down_10(65535, 50), 50);
    assert_eq!(round_down_10(100, 101), 100);
    assert_eq!(round_down_10(65535, 101), 100);
    assert_eq!(round_down_10(255, 256), 250);
    assert_eq!(round_down_10(65535, 256), 250);
    assert_eq!(round_down_10(1, 0), 0);
    assert_eq!(round_down_10(0, 1), 0);
    assert_eq!(round_down_10(0, 0), 0);
    assert_eq!(round_down_10(1, 1), 0);
    assert_eq!(round_down_10(9, 9), 0);
    assert_eq!(round_down_10(10, 10), 10);
    assert_eq!(round_down_10(11, 11), 10);
    assert_eq!(round_down_10(12, 12), 10);
    assert_eq!(round_down_10(13, 13), 10);
    assert_eq!(round_down_10(20, 20), 20);
    assert_eq!(round_down_10(21, 21), 20);
    assert_eq!(round_down_10(100, 100), 100);
}

#[test]
fn submission_marker() {
    assert_eq!(task_153::TRACK_PROOF, 0x52540000 | 153);
}

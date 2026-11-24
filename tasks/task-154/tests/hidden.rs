use task_154::*;

#[test]
fn edge_cases() {
    assert_eq!(round_up_10(65535, 0), 0);
    assert_eq!(round_up_10(1, 2), 10);
    assert_eq!(round_up_10(65535, 9), 10);
    assert_eq!(round_up_10(10, 11), 10);
    assert_eq!(round_up_10(65535, 11), 20);
    assert_eq!(round_up_10(65535, 65535), 65540);
    assert_eq!(round_up_10(65535, 20), 20);
    assert_eq!(round_up_10(40, 41), 40);
    assert_eq!(round_up_10(65535, 50), 50);
    assert_eq!(round_up_10(100, 101), 100);
    assert_eq!(round_up_10(65535, 101), 110);
    assert_eq!(round_up_10(255, 256), 260);
    assert_eq!(round_up_10(65535, 256), 260);
    assert_eq!(round_up_10(1, 0), 0);
    assert_eq!(round_up_10(0, 1), 0);
    assert_eq!(round_up_10(0, 0), 0);
    assert_eq!(round_up_10(1, 1), 10);
    assert_eq!(round_up_10(9, 9), 10);
    assert_eq!(round_up_10(10, 10), 10);
    assert_eq!(round_up_10(11, 11), 20);
    assert_eq!(round_up_10(12, 12), 20);
    assert_eq!(round_up_10(13, 13), 20);
    assert_eq!(round_up_10(20, 20), 20);
    assert_eq!(round_up_10(21, 21), 30);
    assert_eq!(round_up_10(100, 100), 100);
}

#[test]
fn submission_marker() {
    assert_eq!(task_154::TRACK_PROOF, 0x52540000 | 154);
}

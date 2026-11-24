use task_114::*;

#[test]
fn edge_cases() {
    assert_eq!(round_up_2(0), 0);
    assert_eq!(round_up_2(1), 2);
    assert_eq!(round_up_2(1), 2);
    assert_eq!(round_up_2(2), 2);
    assert_eq!(round_up_2(3), 4);
    assert_eq!(round_up_2(65535), 65536);
    assert_eq!(round_up_2(20), 20);
    assert_eq!(round_up_2(40), 40);
    assert_eq!(round_up_2(50), 50);
    assert_eq!(round_up_2(100), 100);
    assert_eq!(round_up_2(101), 102);
    assert_eq!(round_up_2(255), 256);
    assert_eq!(round_up_2(256), 256);
}

#[test]
fn submission_marker() {
    assert_eq!(task_114::TRACK_PROOF, 0x52540000 | 114);
}

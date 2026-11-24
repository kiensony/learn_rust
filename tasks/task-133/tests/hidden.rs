use task_133::*;

#[test]
fn edge_cases() {
    assert_eq!(round_down_5(0, 2), 0);
    assert_eq!(round_down_5(1, 0), 0);
    assert_eq!(round_down_5(4, 1), 0);
    assert_eq!(round_down_5(5, 2), 0);
    assert_eq!(round_down_5(6, 0), 5);
    assert_eq!(round_down_5(65535, 1), 65530);
    assert_eq!(round_down_5(20, 2), 15);
    assert_eq!(round_down_5(40, 0), 40);
    assert_eq!(round_down_5(50, 1), 45);
    assert_eq!(round_down_5(100, 2), 95);
    assert_eq!(round_down_5(101, 0), 100);
    assert_eq!(round_down_5(255, 1), 250);
    assert_eq!(round_down_5(256, 2), 250);
    assert_eq!(round_down_5(0, 9), 0);
    assert_eq!(round_down_5(100, 0), 100);
    assert_eq!(round_down_5(100, 1), 95);
    assert_eq!(round_down_5(0, 0), 0);
    assert_eq!(round_down_5(5, 0), 5);
    assert_eq!(round_down_5(7, 0), 5);
    assert_eq!(round_down_5(8, 0), 5);
    assert_eq!(round_down_5(10, 0), 10);
    assert_eq!(round_down_5(11, 0), 10);
    assert_eq!(round_down_5(20, 0), 20);
}

#[test]
fn submission_marker() {
    assert_eq!(task_133::TRACK_PROOF, 0x52540000 | 133);
}

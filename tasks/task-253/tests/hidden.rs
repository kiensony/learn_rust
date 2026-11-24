use task_253::*;

#[test]
fn edge_cases() {
    assert_eq!(running_deposits_10(255, 0), 0);
    assert_eq!(running_deposits_10(1, 2), 10);
    assert_eq!(running_deposits_10(255, 9), 450);
    assert_eq!(running_deposits_10(10, 11), 550);
    assert_eq!(running_deposits_10(255, 11), 660);
    assert_eq!(running_deposits_10(255, 255), 326400);
    assert_eq!(running_deposits_10(1, 0), 0);
    assert_eq!(running_deposits_10(0, 1), 0);
    assert_eq!(running_deposits_10(0, 0), 0);
    assert_eq!(running_deposits_10(1, 1), 10);
    assert_eq!(running_deposits_10(9, 9), 450);
    assert_eq!(running_deposits_10(10, 10), 550);
    assert_eq!(running_deposits_10(11, 11), 660);
    assert_eq!(running_deposits_10(12, 12), 780);
    assert_eq!(running_deposits_10(13, 13), 910);
    assert_eq!(running_deposits_10(20, 20), 2100);
    assert_eq!(running_deposits_10(21, 21), 2310);
    assert_eq!(running_deposits_10(100, 100), 50500);
}

#[test]
fn submission_marker() {
    assert_eq!(task_253::TRACK_PROOF, 0x52540000 | 253);
}

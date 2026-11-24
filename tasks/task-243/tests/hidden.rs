use task_243::*;

#[test]
fn edge_cases() {
    assert_eq!(running_deposits_8(0, 2), 24);
    assert_eq!(running_deposits_8(1, 3), 24);
    assert_eq!(running_deposits_8(7, 4), 48);
    assert_eq!(running_deposits_8(8, 5), 48);
    assert_eq!(running_deposits_8(9, 6), 48);
    assert_eq!(running_deposits_8(255, 0), 261120);
    assert_eq!(running_deposits_8(18, 3), 960);
    assert_eq!(running_deposits_8(3, 18), 960);
    assert_eq!(running_deposits_8(4, 4), 0);
    assert_eq!(running_deposits_8(0, 0), 0);
    assert_eq!(running_deposits_8(1, 0), 8);
    assert_eq!(running_deposits_8(7, 0), 224);
    assert_eq!(running_deposits_8(8, 0), 288);
    assert_eq!(running_deposits_8(9, 0), 360);
    assert_eq!(running_deposits_8(10, 0), 440);
    assert_eq!(running_deposits_8(11, 0), 528);
    assert_eq!(running_deposits_8(16, 0), 1088);
    assert_eq!(running_deposits_8(17, 0), 1224);
    assert_eq!(running_deposits_8(20, 0), 1680);
    assert_eq!(running_deposits_8(100, 0), 40400);
}

#[test]
fn submission_marker() {
    assert_eq!(task_243::TRACK_PROOF, 0x52540000 | 243);
}

use task_233::*;

#[test]
fn edge_cases() {
    assert_eq!(running_deposits_5(0, 2), 0);
    assert_eq!(running_deposits_5(1, 0), 5);
    assert_eq!(running_deposits_5(4, 1), 30);
    assert_eq!(running_deposits_5(5, 2), 30);
    assert_eq!(running_deposits_5(6, 0), 105);
    assert_eq!(running_deposits_5(255, 1), 161925);
    assert_eq!(running_deposits_5(0, 9), 0);
    assert_eq!(running_deposits_5(100, 0), 25250);
    assert_eq!(running_deposits_5(100, 1), 24750);
    assert_eq!(running_deposits_5(0, 0), 0);
    assert_eq!(running_deposits_5(5, 0), 75);
    assert_eq!(running_deposits_5(7, 0), 140);
    assert_eq!(running_deposits_5(8, 0), 180);
    assert_eq!(running_deposits_5(10, 0), 275);
    assert_eq!(running_deposits_5(11, 0), 330);
    assert_eq!(running_deposits_5(20, 0), 1050);
}

#[test]
fn submission_marker() {
    assert_eq!(task_233::TRACK_PROOF, 0x52540000 | 233);
}

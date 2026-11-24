use task_223::*;

#[test]
fn edge_cases() {
    assert_eq!(running_deposits_3(0, 6), 63);
    assert_eq!(running_deposits_3(1, 9), 165);
    assert_eq!(running_deposits_3(2, 12), 315);
    assert_eq!(running_deposits_3(3, 15), 513);
    assert_eq!(running_deposits_3(4, 1), 45);
    assert_eq!(running_deposits_3(255, 4), 97920);
    assert_eq!(running_deposits_3(255, 1), 97920);
    assert_eq!(running_deposits_3(0, 0), 0);
    assert_eq!(running_deposits_3(1, 0), 3);
    assert_eq!(running_deposits_3(2, 0), 9);
    assert_eq!(running_deposits_3(3, 0), 18);
    assert_eq!(running_deposits_3(5, 0), 45);
    assert_eq!(running_deposits_3(6, 0), 63);
    assert_eq!(running_deposits_3(7, 0), 84);
    assert_eq!(running_deposits_3(20, 0), 630);
    assert_eq!(running_deposits_3(100, 0), 15150);
}

#[test]
fn submission_marker() {
    assert_eq!(task_223::TRACK_PROOF, 0x52540000 | 223);
}

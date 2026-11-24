use task_122::*;

#[test]
fn edge_cases() {
    assert_eq!(price_in_cents_3(0, 6), 18);
    assert_eq!(price_in_cents_3(1, 9), 30);
    assert_eq!(price_in_cents_3(2, 12), 42);
    assert_eq!(price_in_cents_3(3, 15), 54);
    assert_eq!(price_in_cents_3(4, 1), 15);
    assert_eq!(price_in_cents_3(65535, 4), 196605);
    assert_eq!(price_in_cents_3(20, 7), 81);
    assert_eq!(price_in_cents_3(40, 10), 150);
    assert_eq!(price_in_cents_3(50, 13), 189);
    assert_eq!(price_in_cents_3(100, 16), 348);
    assert_eq!(price_in_cents_3(101, 2), 309);
    assert_eq!(price_in_cents_3(255, 5), 780);
    assert_eq!(price_in_cents_3(256, 8), 792);
    assert_eq!(price_in_cents_3(65535, 1), 196605);
    assert_eq!(price_in_cents_3(0, 0), 0);
    assert_eq!(price_in_cents_3(1, 0), 3);
    assert_eq!(price_in_cents_3(2, 0), 6);
    assert_eq!(price_in_cents_3(3, 0), 9);
    assert_eq!(price_in_cents_3(5, 0), 15);
    assert_eq!(price_in_cents_3(6, 0), 18);
    assert_eq!(price_in_cents_3(7, 0), 21);
    assert_eq!(price_in_cents_3(20, 0), 60);
    assert_eq!(price_in_cents_3(100, 0), 300);
}

#[test]
fn submission_marker() {
    assert_eq!(task_122::TRACK_PROOF, 0x52540000 | 122);
}

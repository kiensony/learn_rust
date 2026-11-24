use task_132::*;

#[test]
fn edge_cases() {
    assert_eq!(price_in_cents_5(0, 2), 0);
    assert_eq!(price_in_cents_5(1, 0), 5);
    assert_eq!(price_in_cents_5(4, 1), 15);
    assert_eq!(price_in_cents_5(5, 2), 15);
    assert_eq!(price_in_cents_5(6, 0), 30);
    assert_eq!(price_in_cents_5(65535, 1), 327670);
    assert_eq!(price_in_cents_5(20, 2), 90);
    assert_eq!(price_in_cents_5(40, 0), 200);
    assert_eq!(price_in_cents_5(50, 1), 245);
    assert_eq!(price_in_cents_5(100, 2), 490);
    assert_eq!(price_in_cents_5(101, 0), 505);
    assert_eq!(price_in_cents_5(255, 1), 1270);
    assert_eq!(price_in_cents_5(256, 2), 1270);
    assert_eq!(price_in_cents_5(0, 9), 0);
    assert_eq!(price_in_cents_5(100, 0), 500);
    assert_eq!(price_in_cents_5(100, 1), 495);
    assert_eq!(price_in_cents_5(0, 0), 0);
    assert_eq!(price_in_cents_5(5, 0), 25);
    assert_eq!(price_in_cents_5(7, 0), 35);
    assert_eq!(price_in_cents_5(8, 0), 40);
    assert_eq!(price_in_cents_5(10, 0), 50);
    assert_eq!(price_in_cents_5(11, 0), 55);
    assert_eq!(price_in_cents_5(20, 0), 100);
}

#[test]
fn submission_marker() {
    assert_eq!(task_132::TRACK_PROOF, 0x52540000 | 132);
}

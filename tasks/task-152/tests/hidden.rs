use task_152::*;

#[test]
fn edge_cases() {
    assert_eq!(price_in_cents_10(65535, 0), 0);
    assert_eq!(price_in_cents_10(1, 2), 10);
    assert_eq!(price_in_cents_10(65535, 9), 90);
    assert_eq!(price_in_cents_10(10, 11), 100);
    assert_eq!(price_in_cents_10(65535, 11), 110);
    assert_eq!(price_in_cents_10(65535, 65535), 655350);
    assert_eq!(price_in_cents_10(65535, 20), 200);
    assert_eq!(price_in_cents_10(40, 41), 400);
    assert_eq!(price_in_cents_10(65535, 50), 500);
    assert_eq!(price_in_cents_10(100, 101), 1000);
    assert_eq!(price_in_cents_10(65535, 101), 1010);
    assert_eq!(price_in_cents_10(255, 256), 2550);
    assert_eq!(price_in_cents_10(65535, 256), 2560);
    assert_eq!(price_in_cents_10(1, 0), 0);
    assert_eq!(price_in_cents_10(0, 1), 0);
    assert_eq!(price_in_cents_10(0, 0), 0);
    assert_eq!(price_in_cents_10(1, 1), 10);
    assert_eq!(price_in_cents_10(9, 9), 90);
    assert_eq!(price_in_cents_10(10, 10), 100);
    assert_eq!(price_in_cents_10(11, 11), 110);
    assert_eq!(price_in_cents_10(12, 12), 120);
    assert_eq!(price_in_cents_10(13, 13), 130);
    assert_eq!(price_in_cents_10(20, 20), 200);
    assert_eq!(price_in_cents_10(21, 21), 210);
    assert_eq!(price_in_cents_10(100, 100), 1000);
}

#[test]
fn submission_marker() {
    assert_eq!(task_152::TRACK_PROOF, 0x52540000 | 152);
}

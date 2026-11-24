use task_259::*;

#[test]
fn edge_cases() {
    assert_eq!(number_of_divisors_10(255, 0), 10);
    assert_eq!(number_of_divisors_10(1, 2), 1);
    assert_eq!(number_of_divisors_10(255, 9), 3);
    assert_eq!(number_of_divisors_10(10, 11), 4);
    assert_eq!(number_of_divisors_10(255, 11), 1);
    assert_eq!(number_of_divisors_10(255, 255), 3);
    assert_eq!(number_of_divisors_10(1, 0), 10);
    assert_eq!(number_of_divisors_10(0, 1), 10);
    assert_eq!(number_of_divisors_10(0, 0), 10);
    assert_eq!(number_of_divisors_10(1, 1), 1);
    assert_eq!(number_of_divisors_10(9, 9), 3);
    assert_eq!(number_of_divisors_10(10, 10), 4);
    assert_eq!(number_of_divisors_10(11, 11), 1);
    assert_eq!(number_of_divisors_10(12, 12), 5);
    assert_eq!(number_of_divisors_10(13, 13), 1);
    assert_eq!(number_of_divisors_10(20, 20), 5);
    assert_eq!(number_of_divisors_10(21, 21), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_259::TRACK_PROOF, 0x52540000 | 259);
}

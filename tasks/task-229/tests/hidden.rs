use task_229::*;

#[test]
fn edge_cases() {
    assert_eq!(number_of_divisors_3(13, 3), 2);
    assert_eq!(number_of_divisors_3(1, 9), 2);
    assert_eq!(number_of_divisors_3(2, 12), 2);
    assert_eq!(number_of_divisors_3(3, 15), 3);
    assert_eq!(number_of_divisors_3(4, 1), 1);
    assert_eq!(number_of_divisors_3(255, 4), 2);
    assert_eq!(number_of_divisors_3(255, 1), 2);
    assert_eq!(number_of_divisors_3(0, 0), 3);
    assert_eq!(number_of_divisors_3(1, 0), 1);
    assert_eq!(number_of_divisors_3(2, 0), 2);
    assert_eq!(number_of_divisors_3(3, 0), 2);
    assert_eq!(number_of_divisors_3(5, 0), 1);
    assert_eq!(number_of_divisors_3(6, 0), 3);
    assert_eq!(number_of_divisors_3(7, 0), 1);
    assert_eq!(number_of_divisors_3(20, 0), 2);
    assert_eq!(number_of_divisors_3(100, 0), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_229::TRACK_PROOF, 0x52540000 | 229);
}

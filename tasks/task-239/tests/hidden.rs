use task_239::*;

#[test]
fn edge_cases() {
    assert_eq!(number_of_divisors_5(0, 2), 5);
    assert_eq!(number_of_divisors_5(1, 0), 1);
    assert_eq!(number_of_divisors_5(4, 1), 2);
    assert_eq!(number_of_divisors_5(5, 2), 2);
    assert_eq!(number_of_divisors_5(6, 0), 3);
    assert_eq!(number_of_divisors_5(255, 1), 2);
    assert_eq!(number_of_divisors_5(0, 9), 5);
    assert_eq!(number_of_divisors_5(100, 0), 4);
    assert_eq!(number_of_divisors_5(100, 1), 2);
    assert_eq!(number_of_divisors_5(0, 0), 5);
    assert_eq!(number_of_divisors_5(5, 0), 2);
    assert_eq!(number_of_divisors_5(7, 0), 1);
    assert_eq!(number_of_divisors_5(8, 0), 3);
    assert_eq!(number_of_divisors_5(10, 0), 3);
    assert_eq!(number_of_divisors_5(11, 0), 1);
    assert_eq!(number_of_divisors_5(20, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_239::TRACK_PROOF, 0x52540000 | 239);
}

use task_249::*;

#[test]
fn edge_cases() {
    assert_eq!(number_of_divisors_8(0, 2), 2);
    assert_eq!(number_of_divisors_8(1, 3), 2);
    assert_eq!(number_of_divisors_8(7, 4), 2);
    assert_eq!(number_of_divisors_8(8, 5), 2);
    assert_eq!(number_of_divisors_8(9, 6), 2);
    assert_eq!(number_of_divisors_8(255, 0), 3);
    assert_eq!(number_of_divisors_8(18, 3), 3);
    assert_eq!(number_of_divisors_8(3, 18), 3);
    assert_eq!(number_of_divisors_8(4, 4), 8);
    assert_eq!(number_of_divisors_8(0, 0), 8);
    assert_eq!(number_of_divisors_8(1, 0), 1);
    assert_eq!(number_of_divisors_8(7, 0), 2);
    assert_eq!(number_of_divisors_8(8, 0), 4);
    assert_eq!(number_of_divisors_8(9, 0), 2);
    assert_eq!(number_of_divisors_8(10, 0), 3);
    assert_eq!(number_of_divisors_8(11, 0), 1);
    assert_eq!(number_of_divisors_8(16, 0), 4);
    assert_eq!(number_of_divisors_8(17, 0), 1);
    assert_eq!(number_of_divisors_8(20, 0), 4);
    assert_eq!(number_of_divisors_8(100, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_249::TRACK_PROOF, 0x52540000 | 249);
}

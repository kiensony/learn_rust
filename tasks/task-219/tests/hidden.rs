use task_219::*;

#[test]
fn edge_cases() {
    assert_eq!(number_of_divisors_2(0), 2);
    assert_eq!(number_of_divisors_2(1), 1);
    assert_eq!(number_of_divisors_2(1), 1);
    assert_eq!(number_of_divisors_2(2), 2);
    assert_eq!(number_of_divisors_2(3), 1);
    assert_eq!(number_of_divisors_2(255), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_219::TRACK_PROOF, 0x52540000 | 219);
}

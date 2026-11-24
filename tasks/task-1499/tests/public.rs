use task_1499::*;

#[test]
fn public_examples() {
    assert_eq!(
        response_boundaries_frequent_readings(" 302 204 503 400 201 ", 3),
        vec![0u32, 1u32, 4u32]
    );
    assert_eq!(response_boundaries_frequent_readings("", 0), vec![]);
}

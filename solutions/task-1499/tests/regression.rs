use task_1499::*;

#[test]
fn transfer_case() {
    assert_eq!(
        response_boundaries_frequent_readings(" 302 204 503 400 201   302 204 503 400 201 ", 2),
        vec![0u32, 1u32]
    );
}

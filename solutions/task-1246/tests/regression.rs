use task_1246::*;

#[test]
fn transfer_case() {
    assert_eq!(
        line_byte_lengths_threshold_crossing("nine\nq\nthree\nx\n nine\nq\nthree\nx\n", 1),
        Some((0, 4))
    );
}

use task_1500::*;

#[test]
fn transfer_case() {
    assert_eq!(
        status_detail_replay_mismatch(
            " 302 204 503 400 201   302 204 503 400 201 ",
            &[2, 4, 3, 0, 1, 2, 4, 3, 0, 1]
        ),
        None
    );
}

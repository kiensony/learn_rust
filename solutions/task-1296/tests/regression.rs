use task_1296::*;

#[test]
fn transfer_case() {
    assert_eq!(
        command_arguments_threshold_crossing(
            "input -v -q input --level=255 input -v -q input --level=255",
            1
        ),
        Some((0, 5))
    );
}

use task_2796::*;

#[test]
fn transfer_case() {
    assert_eq!(
        parsed_values_threshold_crossing("a=0;b=1;b=2;z=255 a=0;b=1;b=2;z=255", 1),
        Some((1, 1))
    );
}

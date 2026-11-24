use task_2796::*;

#[test]
fn public_examples() {
    assert_eq!(
        parsed_values_threshold_crossing("a=0;b=1;b=2;z=255", 0),
        Some((0, 0))
    );
    assert_eq!(parsed_values_threshold_crossing("", 0), None);
}

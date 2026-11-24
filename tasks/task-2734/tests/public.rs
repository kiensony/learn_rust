use task_2734::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_peaks(""), vec![]);
    assert_eq!(cli_rust_sources_peaks("a.txt"), vec![]);
}

use task_1048::*;

#[test]
fn public_examples() {
    assert_eq!(zip_array([2, 4], ["a", "b"]), [(2, "a"), (4, "b")]);
    assert_eq!(zip_array([true], [3]), [(true, 3)]);
}

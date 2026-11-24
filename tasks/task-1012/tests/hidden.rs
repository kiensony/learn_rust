use task_1012::*;

#[test]
fn edge_cases() {
    assert_eq!(swap_pair((false, true)), (true, false));
    assert_eq!(
        swap_pair((String::from("a"), String::from("b"))),
        (String::from("b"), String::from("a"))
    );
    assert_eq!(swap_pair(((1, 2), (4, 5))), ((4, 5), (1, 2)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1012::TRACK_PROOF, 0x52540000 | 1012);
}

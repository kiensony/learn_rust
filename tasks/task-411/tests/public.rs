use task_411::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("hi:kit");
            closing_marks_removed_colon(&mut data);
            data
        },
        "hi:kit"
    );
    assert_eq!(
        {
            let mut data = String::from(":é:a");
            closing_marks_removed_colon(&mut data);
            data
        },
        ":é:a"
    );
}

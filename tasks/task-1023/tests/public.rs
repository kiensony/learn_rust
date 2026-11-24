use task_1023::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe(["first\n123", "header\n猫|123"]), [3, 3]);
    assert_eq!(closing_field_pipe(["hi|kit"]), [3]);
}

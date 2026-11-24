use task_661::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(Event::Data("")), Some(0));
    assert_eq!(prefix_before_final_mark_colon(Event::Data(":")), Some(0));
    assert_eq!(prefix_before_final_mark_colon(Event::Data("cat")), Some(0));
    assert_eq!(
        prefix_before_final_mark_colon(Event::Data("::one:猫:ab")),
        Some(9)
    );
    assert_eq!(
        prefix_before_final_mark_colon(Event::Data("aa:bb")),
        Some(2)
    );
    assert_eq!(
        prefix_before_final_mark_colon(Event::Data("ok:🐈")),
        Some(2)
    );
    assert_eq!(prefix_before_final_mark_colon(Event::Heartbeat), Some(0));
    assert_eq!(prefix_before_final_mark_colon(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_661::TRACK_PROOF, 0x52540000 | 661);
}

use task_691::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("hi=kit")),
        Some(2)
    );
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("=é=a")),
        Some(3)
    );
    assert_eq!(prefix_before_final_mark_equals(Event::Data("")), Some(0));
    assert_eq!(prefix_before_final_mark_equals(Event::Data("=")), Some(0));
    assert_eq!(prefix_before_final_mark_equals(Event::Data("cat")), Some(0));
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("==one=猫=ab")),
        Some(9)
    );
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("aa=bb")),
        Some(2)
    );
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("ok=🐈")),
        Some(2)
    );
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("aa=bb;meta=123")),
        Some(2)
    );
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data(";ignore=all")),
        Some(0)
    );
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("a==b;")),
        Some(2)
    );
    assert_eq!(prefix_before_final_mark_equals(Event::Heartbeat), Some(0));
    assert_eq!(prefix_before_final_mark_equals(Event::End), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_691::TRACK_PROOF, 0x52540000 | 691);
}

mod common;

#[test]
fn boundaries() {
    common::check(&[], "==  ==\n", "");
    common::check(&["🇯🇵"], "== 🇯🇵 ==\n", "");
    common::check(&[" a "], "==  a  ==\n", "");
}

#[test]
fn reusable_label() {
    assert_eq!(task_030::banner(""), "==  ==");
    assert_eq!(task_030::banner("🇯🇵"), "== 🇯🇵 ==");
    assert_eq!(task_030::banner(" a "), "==  a  ==");
}

#[test]
fn submission_marker() {
    assert_eq!(task_030::TRACK_PROOF, 0x52540000 | 30, "See the submission step in REVIEW.md");
}

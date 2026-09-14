use std::collections::BTreeMap;
use task_008::count_words;

#[test]
fn simple() {
    let mut want = BTreeMap::new();
    want.insert("a".into(), 2);
    want.insert("b".into(), 1);
    assert_eq!(count_words("A a b"), want);
}

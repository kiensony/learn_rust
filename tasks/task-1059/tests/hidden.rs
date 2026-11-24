use task_1059::*;

#[test]
fn boundaries() {
    assert_eq!(Dial(0).read(), 0);
    assert_eq!(
        {
            let d = Dial(255);
            (d.read(), d.read())
        },
        (255, 255)
    );
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1059);
}

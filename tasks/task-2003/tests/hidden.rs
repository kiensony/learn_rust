use std::future::Future;
use task_2003::*;

#[test]
fn boundaries() {
    assert_eq!(
        {
            let x = reading(std::future::ready(0));
            std::pin::pin!(x).poll(&mut std::task::Context::from_waker(std::task::Waker::noop()))
        },
        std::task::Poll::Ready(0)
    );
    assert_eq!(
        {
            let x = reading(std::future::pending());
            std::pin::pin!(x).poll(&mut std::task::Context::from_waker(std::task::Waker::noop()))
        },
        std::task::Poll::Pending
    );
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 2003);
}

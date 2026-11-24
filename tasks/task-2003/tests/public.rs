use std::future::Future;
use task_2003::*;

#[test]
fn example() {
    assert_eq!(
        {
            let x = reading(std::future::ready(7));
            std::pin::pin!(x).poll(&mut std::task::Context::from_waker(std::task::Waker::noop()))
        },
        std::task::Poll::Ready(7)
    );
}

// rust-track: handwritten task-2029
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2029;

pub fn polled_groups_longest_run(input: &[u8]) -> u64 {
    let samples = observe_polled_groups(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_polled_groups(input: &[u8]) -> Vec<u32> {
    use std::future::Future;
    use std::task::{Context, Poll, Waker};
    let mut cx = Context::from_waker(Waker::noop());
    input
        .iter()
        .map(|&x| {
            let mut first = true;
            let f = std::future::poll_fn(|cx| {
                if first {
                    first = false;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                } else {
                    Poll::Ready(u32::from(x / 4))
                }
            });
            let mut pin = std::pin::pin!(f);
            assert!(pin.as_mut().poll(&mut cx).is_pending());
            match pin.as_mut().poll(&mut cx) {
                Poll::Ready(v) => v,
                Poll::Pending => unreachable!(),
            }
        })
        .collect()
}

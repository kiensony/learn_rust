// rust-track: handwritten task-2037
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2037;

pub fn poll_gate(delays: u8) -> (u16, u16) {
    use std::future::{Future, poll_fn};
    use std::sync::{
        Arc,
        atomic::{AtomicU16, Ordering},
    };
    use std::task::{Context, Poll, Wake, Waker};

    struct Counter(AtomicU16);
    impl Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    let count = Arc::new(Counter(AtomicU16::new(0)));
    let waker = Waker::from(Arc::clone(&count));
    let mut cx = Context::from_waker(&waker);
    let mut remaining = delays;
    let future = poll_fn(|cx| {
        if remaining == 0 {
            return Poll::Ready(());
        }
        remaining -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    });
    let mut future = std::pin::pin!(future);
    let mut polls = 0;
    for _ in 0..=delays {
        polls += 1;
        if future.as_mut().poll(&mut cx).is_ready() {
            break;
        }
    }
    (polls, count.0.load(Ordering::Relaxed))
}

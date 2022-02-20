use instant::{Duration, Instant};
use std::cmp::Ordering;
use std::future::Future;
use std::ops::Add;
use std::pin::Pin;
use std::rc::Rc;
use std::rc::Weak;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

use crate::runtime::RuntimeState;

pub(crate) struct Timeout {
    pub(crate) instant: instant::Instant,
    pub(crate) waker: Waker,
}
impl PartialEq for Timeout {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}
impl Eq for Timeout {}
impl Ord for Timeout {
    fn cmp(&self, other: &Timeout) -> Ordering {
        self.instant.cmp(&other.instant).reverse() // for max-heap to min-heap
    }
}
impl PartialOrd for Timeout {
    fn partial_cmp(&self, other: &Timeout) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

pub struct TimerFuture {
    instant: Instant,
    runtime: Weak<RuntimeState>,
}
impl TimerFuture {
    pub(crate) fn new(executor: &Rc<RuntimeState>, delay: Duration) -> Self {
        Self {
            instant: executor.current_time.borrow().add(delay),
            runtime: Rc::downgrade(executor),
        }
    }
}
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(executor) = self.runtime.upgrade() {
            if self.instant < *executor.current_time.borrow() {
                Poll::Ready(())
            } else {
                executor.timer_waker_heap.borrow_mut().push(Timeout {
                    instant: self.instant,
                    waker: cx.waker().clone(),
                });
                Poll::Pending
            }
        } else {
            unreachable!()
        }
    }
}

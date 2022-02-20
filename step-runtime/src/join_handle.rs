use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

pub struct JoinHandle<T> {
    pub(crate) value: Rc<RefCell<Option<T>>>,
    pub(crate) register_handle_waker: Box<dyn Fn(Waker) -> ()>,
}
impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(val) = self.value.borrow_mut().take() {
            Poll::Ready(val)
        } else {
            let waker = cx.waker().clone();
            (self.register_handle_waker)(waker);
            Poll::Pending
        }
    }
}

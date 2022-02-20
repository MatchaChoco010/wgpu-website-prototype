use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};
use uuid::Uuid;

use crate::JoinHandle;

pub(crate) struct Task {
    pub(crate) id: Uuid,
    pub(crate) future: Pin<Box<dyn Future<Output = ()>>>,
    pub(crate) handle_waker: Rc<RefCell<Option<Waker>>>,
}
impl Task {
    pub(crate) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        Future::poll(self.future.as_mut(), cx)
    }
}

pub(crate) fn joinable<F>(future: F) -> (Task, JoinHandle<F::Output>)
where
    F: Future + 'static,
    F::Output: 'static,
{
    let value = Rc::new(RefCell::new(None));

    let task = {
        let value = Rc::clone(&value);
        Task {
            future: Box::pin(async move {
                let output = future.await;
                value.borrow_mut().replace(output);
            }),
            handle_waker: Rc::new(RefCell::new(None)),
            id: Uuid::new_v4(),
        }
    };

    let register_handle_waker = Box::new({
        let handle_waker = Rc::clone(&task.handle_waker);
        move |waker| {
            handle_waker.borrow_mut().replace(waker);
        }
    });
    let handle = JoinHandle {
        value,
        register_handle_waker,
    };

    (task, handle)
}
